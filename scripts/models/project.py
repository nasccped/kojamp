from colors import GREEN_NONE, RESET_ESCAPE
from error_types.base_error import BaseError
from models.file import File
from models.dockerhub_bridge import DockerHubBridge
from models.crates_io_bridge import CratesIOBridge
from models.local_git_bridge import LocalGitBridge
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
        r_git_bridge: RemoteGitBridge,
        l_git_bridge: LocalGitBridge
    ) -> None:
        # unwrap errors in a list
        errors = [
            file.unwrap_err(),
            dockerhub_bridge.unwrap_err(),
            crates_io_bridge.unwrap_err(),
            r_git_bridge.unwrap_err(),
            l_git_bridge.unwrap_err()
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
        self.remote = r_git_bridge
        self.local = l_git_bridge
        self.errors = errors

    def get_error_list(self) -> None | list[BaseError]:
        return self.errors

    def print_versions(self):
        versions = [
            v.__str__(
                GREEN_NONE + c.__class__.__name__ + RESET_ESCAPE
            ) for (v, c) in [
                (self.file.version, self.file),
                (self.dhub.latest, self.dhub),
                (self.crate.latest, self.crate),
                (self.remote.latest, self.remote),
                (self.local.latest, self.local)
            ]
        ]

        print("\n".join(versions))
