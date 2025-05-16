import sys

class BaseError:

    def __init__(self, message: str, exit_stts: int = 1) -> None:
        self.error_message = message
        self.exit_status   = exit_stts

    def print_message(self) -> None:
        print(self.error_message)

    def exit_with_status(self) -> None:
        sys.exit(self.exit_status)
