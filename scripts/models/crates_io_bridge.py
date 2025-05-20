import json
import base64
from core.urls import CRATES_IO_URL
from core.env import CRATE_NAME, GITHUB_TOKEN_PATH
from error_types.derived_errors import UnfetchableURL
from error_types.derived_errors import UnfetchableFileData
from models.program_version import ProgramVersion
import requests
from utils.file import read_file_else_none

def get_crateghub_url(target_crate: str) -> str | UnfetchableURL:
    """
    All Rust's crates are disposed at crates.io url and you can
    access it through an url (getting a json).

    To get the full path of a given crate, you'll need to find
    it by navigation by a url tree (nested directories).

    This function will try to build the correct url path to a given
    crate by using brute force. Will return an `UnfetchableURL` if
    no url found.
    """
    # crate url is built like:
    # somecrate => https://.../contents/so/me/somecrate
    # basically, the fields before the crate name will be
    # the crate chars separated by pairs ('so', 'me', '..')
    contents_url = CRATES_IO_URL + "/contents"
    url_parts = [
        target_crate[i : i + 2] \
            for i in range(0, len(target_crate), 2)
    ]
    my_token = "" \
        if (content := read_file_else_none(GITHUB_TOKEN_PATH)) is None \
        else content[0].replace("\n", "")
    headers = {
        "Authorization": f"token {my_token}",
        "Accept": "application/vnd.github.v3+json"
    }

    # while there's items within url parts
    while len(url_parts) > 0:
        path_url = "/".join(url_parts)
        full_crate_url = "/".join(
            [contents_url, path_url, target_crate]
        )
        conn = requests.get(full_crate_url, headers = headers)

        # if url returned OK
        if conn.status_code == 200:
            return full_crate_url

        if len(url_parts[-1]) > 1:
            url_parts[-1] = url_parts[-1][:-1]
            continue

        url_parts.pop()

    # else (no ok url)
    return UnfetchableURL(contents_url)

def get_versionlist_from_crateghub_url(
    url: str
) -> list[str] | UnfetchableURL | UnfetchableFileData:
    """
    Extract `list[str]` of an crates.io json content.

    Will return `UnfetchableURL` if invalid url. Can also return
    `UnfetchableDataFile` if the target field doesn't exists
    """
    conn = requests.get(url)
    target_conn_field = "content"
    target_json_field = "vers"
    target_encoding = "utf-8"

    if conn.status_code != 200:
        return UnfetchableURL(url)

    # it comes encoded in base64. Let's decode it to utf-8
    encoded = conn.json()[target_conn_field]
    decoded = base64.b64decode(encoded)
    data_rows = [
        json.loads(row) \
            for row in decoded.decode(target_encoding).split("\n") \
            if len(row) > 0
    ]

    # use try to avoid FieldDoesntExists error like
    try:
        returnable = [row[target_json_field] for row in data_rows]
    except:
        returnable = UnfetchableFileData(
            f"{url}[\"{target_conn_field}\"]",
            f"{{ {target_json_field} }}"
        )

    return returnable

class CratesIOBridge:
    """
    Store the project info related fields (latest version) from
    crates.io
    """

    def __init__(self, crate_name: str) -> None:
        final_url = get_crateghub_url(CRATE_NAME)
        versions = None

        self.crate_name = crate_name
        self.latest = None
        self.error = None

        # if url catching returned an error, update error field and
        # stop construction
        if isinstance(final_url, UnfetchableURL):
            self.error = final_url
            return

        # extract version list
        result = get_versionlist_from_crateghub_url(final_url)

        # if is error
        if isinstance(result, UnfetchableURL | UnfetchableFileData):
            self.error = result
            return

        # update `latest` field
        versions = [ProgramVersion(v) for v in result]
        versions.sort()
        self.latest = versions[-1]

    def unwrap_err(self) -> None | UnfetchableURL | UnfetchableFileData:
        return self.error
