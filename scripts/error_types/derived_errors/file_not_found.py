from colors import RED_NONE, RESET_ESCAPE, CYAN_NONE
from error_types.base_error import BaseError

class FileNotFound(BaseError):
    """
    Base error when trying to get some file/file content and it
    doesn't exists.
    """

    def __init__(self, file_name: str) -> None:
        red_file = RED_NONE + file_name + RESET_ESCAPE
        repository_root_path = CYAN_NONE + "repository root path" + RESET_ESCAPE
        wrong_branch = RED_NONE + "wrong branch" + RESET_ESCAPE
        permission_issues = RED_NONE + "permission issues" + RESET_ESCAPE
        message = [
            f"Couldn't find the `{red_file}` file!",
            "",
            f"You should run this script in the {repository_root_path}.",
            f"You may be on the {wrong_branch} too.",
            f"This can be due to {permission_issues}, btw."
        ]
        super().__init__(message)
