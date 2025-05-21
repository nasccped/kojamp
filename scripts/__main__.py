"""
This is the publish script main file!

It should be ran in the repository root path.
You don't need to specify the file entry. Just use
`python <PATH_NAME> (scripts for this case)`
"""
from colors import RESET_ESCAPE, GREEN_NONE
from core.env import CRATE_NAME, IMAGE_NAME, REMOTE_REPOSITORY
from core.target_files import CARGO_TOML
from error_types.base_error import BaseError
from models.crates_io_bridge import CratesIOBridge
from models.dockerhub_bridge import DockerHubBridge
from models.file import File
from models.project import Project
from models.remote_git_bridge import RemoteGitBridge
from visual.alerts import waiting_alert
from visual.banner import program_begin, program_errors

def load_models():
    global \
        docker_bridge, \
        crates_bridge, \
        cargo_file, \
        r_git_bridge

    docker_bridge = DockerHubBridge(IMAGE_NAME)
    crates_bridge = CratesIOBridge(CRATE_NAME)
    cargo_file = File(CARGO_TOML)
    r_git_bridge = RemoteGitBridge(REMOTE_REPOSITORY)

def print_script_banner():
    program_begin()

def print_errors(errors: list[BaseError]):
    program_errors()
    for e in errors:
        e.print_content()

def main():
    # TODO: to implement
    print(GREEN_NONE + "To implement..." + RESET_ESCAPE)

if __name__ == "__main__":
    waiting_alert()

    load_models()
    # load project from global fields
    project = Project(cargo_file, docker_bridge, crates_bridge, r_git_bridge)
    # if any error found (exit with status)
    if errs := project.get_error_list():
        print_errors(errs)
        BaseError.exit_with_status(1)

    main()
