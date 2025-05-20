from colors import RED_NONE, RESET_ESCAPE, GREEN_NONE
from error_types.base_error import BaseError

class UnfetchableFileData(BaseError):
    """
    Base error when trying to fetch data field from a given file path
    and it returns an error (file/field doesn't exists)
    """

    def __init__(self, file_name: str, field: str) -> None:
        red_file = RED_NONE + file_name + RESET_ESCAPE
        green_field = GREEN_NONE + field + RESET_ESCAPE
        missing = RED_NONE + "missing" + RESET_ESCAPE
        message = [
            f"Couldn't fetch data from the '{red_file}' file!",
            f"The '{green_field}' field is {missing}."
        ]
        super().__init__(message, 1)
