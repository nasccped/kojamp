from colors import RED_NONE, RESET_ESCAPE, CYAN_NONE
from error_types.base_error import BaseError

class UnfetchableURL(BaseError):
    """
    Base error when trying to fetch data from a given url and it
    returns an error (`status_code` != 200)
    """

    def __init__(self, url: str, stts_code: int) -> None:
        red_url = RED_NONE + url + RESET_ESCAPE
        status_code = CYAN_NONE + str(stts_code) + RESET_ESCAPE
        message = [
            f"Couldn't fetch data from the",
            f"'{red_url}' url!",
            "",
            f"Returned status code {status_code}."
        ]
        super().__init__(message, 1)
