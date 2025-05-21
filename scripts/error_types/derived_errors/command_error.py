from colors import apply, GREEN_NONE
from error_types.base_error import BaseError

class CommandError(BaseError):

    def __init__(self, command: str, args: list[str]):
        green_command = apply(command, GREEN_NONE)
        green_args = [apply(a, GREEN_NONE) for a in args]
        message = [
            f"The {green_command} command returned a non 0 status",
            f"code by using the following args: [{', '.join(green_args)}]",
            "",
            f"Check if the program exists (or is able to be runned)!"
        ]
        super().__init__(message, 1)

