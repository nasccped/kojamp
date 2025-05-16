from colors import RED_NONE, RESET_ESCAPE, CYAN_NONE
from error_types.base_error import BaseError
from models.file import File

class FileNotFound(BaseError):
    """
    Base error when trying to get some file/file content and it
    doesn't exists.
    """

    def __init__(self, message: str | list[str], exit: int = 1) -> None:
        super().__init__(message, exit)

    @classmethod
    def from_file(cls, file: File):
        """
        Create the FileNotFound Object from a File Object.
        """
        red_file = RED_NONE + file.get_file_name() + RESET_ESCAPE
        message = [
            f"Couldn't find the `{red_file}` file!",
            "",
            f"You should run this script in the {CYAN_NONE}repository root path{RESET_ESCAPE}.",
            f"You may be on the {RED_NONE}wrong branch{RESET_ESCAPE} too.",
        ]
        return cls(message)
