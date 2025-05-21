from error_types.base_error import BaseError
from models.file import File
from models.dockerhub_bridge import DockerHubBridge
from models.crates_io_bridge import CratesIOBridge
from models.remote_git_bridge import RemoteGitBridge

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

    def __init__(
        self,
        file: File,
        dockerhub_bridge: DockerHubBridge,
        crates_io_bridge: CratesIOBridge,
        r_git_bridge: RemoteGitBridge
    ) -> None:
        # unwrap errors in a list
        errors = [
            file.unwrap_err(),
            dockerhub_bridge.unwrap_err(),
            crates_io_bridge.unwrap_err(),
            r_git_bridge.unwrap_err()
        ]
        # filter only when not None
        errors = [e for e in errors if e]
        # set None if empty list
        errors = errors \
            if len(errors) > 0 \
            else None

        self.file = file
        self.dhub = dockerhub_bridge
        self.crate = crates_io_bridge
        self.errors = errors

    def get_error_list(self) -> None | list[BaseError]:
        return self.errors
