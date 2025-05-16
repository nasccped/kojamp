from error_types import BaseError

class FileNotFound(BaseError):

    def __init__(self, message: str, exit: int) -> None:
        super().__init__(message, exit)
