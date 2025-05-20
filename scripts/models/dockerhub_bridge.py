from core.urls import DOCKERHUB_URL
from models.program_version import ProgramVersion
from error_types.derived_errors import UnfetchableURL
from error_types.derived_errors import DockerEngineError
from utils.regex import pattern_in_str_sentence
from utils.cmdline import command_is_ok
import requests

def get_docker_latest_tag(url: str) -> ProgramVersion | UnfetchableURL:
    """
    Just take the url json info recursively to extract the tag value
    """
    next_page = url
    tags = []

    while next_page:
        response = requests.get(next_page)
        if response.status_code != 200:
            return UnfetchableURL(response.url)
        data = response.json()
        pat = r"(\d+.\d+.\d)"
        filtered = [d["name"] for d in data["results"]]
        tags.extend(
            ProgramVersion(f) \
                for f in filtered \
                if pattern_in_str_sentence(pat, f)
        )
        next_page = data.get("next")

    tags.sort()
    return tags[-1]

class DockerHubBridge:
    """
    Store the project docker related fields (latest tag,
    specifically)
    """

    def __init__(self, image_name: str) -> None:
        full_url = DOCKERHUB_URL + "/" + image_name
        tags_url = full_url + "/tags"
        latest = get_docker_latest_tag(tags_url)
        error = None

        if isinstance(latest, UnfetchableURL):
            error = latest
            latest = None

        if not command_is_ok("docker", ["images"]):
            error = DockerEngineError()

        self.image_name = image_name
        self.latest = latest
        self.error = error

    def unwrap_err(self) -> None | UnfetchableURL | DockerEngineError:
        return self.error
