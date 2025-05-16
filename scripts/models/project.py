from error_types.base_error import BaseError
from models.file import File
from models.dockerhub_bridge import DockerHubBridge

class Project:
    """
    Class to store the project related fields, such as:

    - versions:
        - version from `Cargo.toml`
        - version from `crates.io`
        - version from `latest docker registry`
        - version from `latest git tag`

    - urls:
        - docker url pushing

    - errors:
        - FileNotFound
        - ErrorUrl
        - ...
    """

    def __init__(self, file: File, dockerhub_bridge: DockerHubBridge) -> None:
        # set them to Project fields
        errors = [file.unwrap_err(), dockerhub_bridge.unwrap_err()]
        errors = [e for e in errors if e]
        self.file: File = file
        self.dhub = dockerhub_bridge
        self.errors = errors \
            if len(errors) > 0 \
            else None

    def get_error_list(self) -> None | list[BaseError]:
        return self.errors
