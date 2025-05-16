from core.urls import DOCKERHUB_URL
from error_types.derived_errors import UnfetchableURL
import requests
import re

def get_docker_tags_list(url: str) -> list[str] | UnfetchableURL:
    """
    Just take the url json info recursively to extract all tag values
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
        tags.extend(f for f in filtered if re.search(pat, f))
        next_page = data.get("next")

    return tags

class DockerHubBridge:
    """
    Store the project docker related fields (latest tag,
    specifically)
    """

    def __init__(self, image_name: str) -> None:
        full_url = DOCKERHUB_URL + "/" + image_name
        tags_url = full_url + "/tags"
        latest_tag = get_docker_tags_list(tags_url)
        error = None

        if isinstance(latest_tag, UnfetchableURL):
            error = latest_tag
            latest_tag = []

        self.image_name = image_name
        self.full_url = full_url
        self.tags_url = tags_url
        self.error = error

    def get_full_url(self) -> str:
        return self.full_url

    def unwrap_err(self) -> None | UnfetchableURL:
        return self.error
