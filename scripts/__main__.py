"""
This is the publish script main file!

It should be ran in the repository root path.
You don't need to specify the file entry. Just use
`python <PATH_NAME> (scripts for this case)`
"""
from colors import CYAN_NONE, RESET_ESCAPE, RED_NONE, GREEN_NONE
from core.env import IMAGE_NAME
from core.target_files import CARGO_TOML
from error_types.base_error import BaseError
from models.dockerhub_bridge import DockerHubBridge
from models.file import File
from models.project import Project
from panel.banner import program_begin, program_errors
from panel.screen import clear

def waiting_alert():
    cyan_files = CYAN_NONE + "files" + RESET_ESCAPE
    cyan_urls = CYAN_NONE + "urls" + RESET_ESCAPE
    take_a_while = RED_NONE + "take a while" + RESET_ESCAPE

    print(f"Fetching data from {cyan_files} and {cyan_urls}!")
    print(f"This can {take_a_while}...")

def load_models():
    global docker_bridge, cargo_file
    docker_bridge = DockerHubBridge(IMAGE_NAME)
    cargo_file = File(CARGO_TOML)

def print_script_banner():
    print()
    program_begin()
    print()

def print_error_banner():
    print()
    program_errors()
    print()

def main():
    # TODO: to implement
    print(GREEN_NONE + "To implement..." + RESET_ESCAPE)

if __name__ == "__main__":
    print()
    waiting_alert()
    print()

    load_models()

    # load project from global fields
    project = Project(cargo_file, docker_bridge)
    # if any error found (exit with status)
    if errs := project.get_error_list():
        print_error_banner()
        for e in errs:
            e.print_content()
        BaseError.exit_with_status(1)

    # else, call main
    main()
