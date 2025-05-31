from colors import apply, RED_NONE, GREEN_NONE
from error_types.base_error import BaseError

class UnfetchableFileData(BaseError):
    """
    Base error when trying to fetch data field from a given file path
    and it returns an error (file/field doesn't exists).
    """

    def __init__(self, file_name: str, field: str) -> None:
        red_file = apply(file_name, RED_NONE)
        green_field = apply(field, GREEN_NONE)
        missing = apply("missing", RED_NONE)
        message = [
            f"Couldn't fetch data from the '{red_file}' file!",
            f"The '{green_field}' field is {missing}."
        ]
        super().__init__(message, 1)
