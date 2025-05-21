from core.env import GITHUB_TOKEN_PATH
from core.urls import GITHUB_API_URL
from models.program_version import ProgramVersion
from error_types.derived_errors import UnfetchableURL
from error_types.derived_errors import UnfetchableFileData
import requests
from utils.file import read_file_else_none

def get_latest_remote_git_tag(
    url: str
) -> ProgramVersion | UnfetchableURL | UnfetchableFileData:
    """
    Get all tags from a remote git branch, sort it and return the
    latest one.
    """
    my_token = content[0].replace("\n", "") \
        if (content := read_file_else_none(GITHUB_TOKEN_PATH)) \
        else ""
    headers = {
        "Authorization": f"token {my_token}",
        "Accept": "application/vnd.github.v3+json"
    }
    conn = requests.get(url, headers=headers)

    if conn.status_code != 200:
        return UnfetchableURL(url, conn.status_code)
    
    latest = None

    try:
        taglist = [ProgramVersion(tag["name"]) for tag in conn.json()]
        taglist.sort()
        latest = taglist.pop()
    except:
        latest = UnfetchableFileData(url, "name")

    return latest


class RemoteGitBridge:
    """
    Store the project data fields from the remote repository
    """

    def __init__(self, remote_repo: str) -> None:
        full_url = GITHUB_API_URL + "/" + remote_repo + "/tags"
        latest = get_latest_remote_git_tag(full_url)
        error = None

        if isinstance(latest, UnfetchableURL):
            error = latest
            latest = None

        self.repo_name = remote_repo
        self.latest = latest
        self.error = error

    def unwrap_err(self) -> None | UnfetchableURL:
        return self.error
