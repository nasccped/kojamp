"""
This is the publish script main file!

It should be ran in the repository root path.
You don't need to specify the file entry. Just use
`python <PATH_NAME> (scripts for this case)`
"""
from core.env import CRATE_NAME, IMAGE_NAME, REMOTE_REPOSITORY
from core.target_files import CARGO_TOML
from error_types.base_error import BaseError
from models.crates_io_bridge import CratesIOBridge
from models.dockerhub_bridge import DockerHubBridge
from models.file import File
from models.local_git_bridge import LocalGitBridge
from models.project import Project
from models.remote_git_bridge import RemoteGitBridge
from visual.alerts import waiting_alert, \
    local_tag_should_be_greater_than_remote, \
    local_tag_conflict, \
    local_version_should_be_greater_than_docker
from visual.banner import init_banner, error_banner

def load_models():
    global \
        docker_bridge, \
        crates_bridge, \
        cargo_file, \
        r_git_bridge, \
        l_git_bridge

    docker_bridge = DockerHubBridge(IMAGE_NAME)
    crates_bridge = CratesIOBridge(CRATE_NAME)
    cargo_file = File(CARGO_TOML)
    r_git_bridge = RemoteGitBridge(REMOTE_REPOSITORY)
    l_git_bridge = LocalGitBridge()

def print_errors_and_exit(errors: list[BaseError], status: int):
    error_banner()
    for e in errors:
        e.print_content()
    BaseError.exit_with_status(status)

def main():
    global project

    local_ver = project.local.latest
    file_ver = project.file.version
    remote_ver = project.remote.latest
    crate_ver = project.crate.latest
    docker_ver = project.dhub.latest

    if local_ver != file_ver:
        local_tag_conflict(str(local_ver), str(file_ver))
        return

    if local_ver <= remote_ver:
        local_tag_should_be_greater_than_remote(
            str(local_ver), str(remote_ver)
        )
        return

    if file_ver <= crate_ver:
        file_version_should_be_greater_than_crate(
            str(file_ver), str(remote_ver)
        )
        return

    if local_ver <= docker_ver:
        local_version_should_be_greater_than_docker(
            str(local_ver), str(docker_ver)
        )
        return
    # TODO: add extra code here vvv


if __name__ == "__main__":
    waiting_alert()

    load_models()
    # load project from global fields
    project = Project(
        cargo_file,
        docker_bridge,
        crates_bridge,
        r_git_bridge,
        l_git_bridge
    )
    # if any error found (exit with status)
    if errs := project.get_error_list():
        print_errors_and_exit(errs, 1)

    init_banner()
    main()
    print()
