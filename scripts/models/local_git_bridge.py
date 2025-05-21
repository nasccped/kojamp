from error_types.derived_errors import CommandError
from models.program_version import ProgramVersion
from error_types.derived_errors import UnfetchableURL
from utils.cmdline import command_is_ok, get_command_output

def get_latest_tag() -> ProgramVersion | CommandError:
    """
    Get the `git tag` command output and returns the latest
    tag from the list
    """
    git_command = "git"
    tag_arg = ["tag"]

    if not command_is_ok(git_command, tag_arg):
        return CommandError(git_command, tag_arg)

    output = get_command_output(git_command, tag_arg)
    tags = [ProgramVersion(t) for t in output]
    tags.sort()
    return tags.pop()

class LocalGitBridge:
    """
    Store the project data fields from the local repository
    """

    def __init__(self) -> None:
        latest = get_latest_tag()
        error = None

        if isinstance(latest, UnfetchableURL):
            error = latest
            latest = None

        self.latest = latest
        self.error = error

    def unwrap_err(self) -> None | UnfetchableURL:
        return self.error
