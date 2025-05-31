from colors import apply, RED_NONE, CYAN_NONE
from error_types.base_error import BaseError

class FileNotFound(BaseError):
    """
    Base error when trying to get some file/file content and it
    doesn't exists.
    """

    def __init__(self, file_name: str) -> None:
        red_file = apply(file_name, RED_NONE)
        repository_root_path = apply(
            "repository root path", CYAN_NONE
        )
        wrong_branch = apply("wrong branch", RED_NONE)
        permission_issues = apply("permission issues", RED_NONE)
        message = [
            f"Couldn't find the `{red_file}` file!",
            "",
            f"You should run this script in the {repository_root_path}.",
            f"You may be on the {wrong_branch} too.",
            f"This can be due to {permission_issues}, btw."
        ]
        super().__init__(message)
