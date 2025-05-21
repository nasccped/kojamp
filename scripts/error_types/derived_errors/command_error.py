from colors import GREEN_NONE, RESET_ESCAPE
from error_types.base_error import BaseError

class CommandError(BaseError):

    def __init__(self, command: str, args: list[str]):
        green_command = GREEN_NONE + command + RESET_ESCAPE
        green_args = [GREEN_NONE + a + RESET_ESCAPE for a in args]
        message = [
            f"The {green_command} command returned a non 0 status",
            f"code by using the following args: [{', '.join(green_args)}]",
            "",
            f"Check if the program exists (or is able to be runned)!"
        ]
        super().__init__(message, 1)

