import json
import base64
from core.urls import CRATES_IO_URL
from error_types.derived_errors import UnfetchableURL
from models.program_version import ProgramVersion
import requests

def get_versions_from_cratesio_ghub(target_crate: str) -> list[str] | UnfetchableURL:
    contents_url = CRATES_IO_URL + "/contents"
    url_parts = [target_crate[i : i + 2] for i in range(0, len(target_crate), 2)]
    conn = None

    while True:
        if len(url_parts) == 0:
            return UnfetchableURL(contents_url)

        path_url = "/".join(url_parts[:-1])
        full_crate_url = "/".join([contents_url, path_url, target_crate])

        conn = requests.get(full_crate_url)

        if conn.status_code == 200:
            break

        if len(url_parts[-1]) > 1:
            url_parts[-1] = url_parts[-1][:-1]
        else:
            url_parts = url_parts[:-1]

    content = conn.json()["content"]
    decoded_bytes = base64.b64decode(content)
    data_rows = decoded_bytes.decode("utf-8").split("\n")

    versions = []

    for row in data_rows:
        if len(row) == 0:
            continue
        as_json = json.loads(row)
        versions.append(as_json["vers"])

    if len(versions) == 0:
        return UnfetchableURL(full_crate_url)

    return versions

class CratesIOBridge:
    """
    Store the project info related fields (latest version) from
    crates.io
    """

    def __init__(self, crate_name: str) -> None:
        versions = get_versions_from_cratesio_ghub(crate_name)

        if isinstance(versions, UnfetchableURL):
            latest = None
            error = versions
        else:
            versions.sort()
            latest = ProgramVersion(versions[-1])
            error = None

        self.crate_name: str = crate_name
        self.latest = latest
        self.error = error

    def unwrap_err(self) -> None | UnfetchableURL:
        return self.error
