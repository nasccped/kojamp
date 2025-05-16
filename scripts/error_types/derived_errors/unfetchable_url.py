from colors import RED_NONE, RESET_ESCAPE, CYAN_NONE
from error_types.base_error import BaseError

class UnfetchableURL(BaseError):
    """
    Base error when trying to fetch data from a given url and it
    returns an error (`status_code` != 200)
    """

    def __init__(self, url: str) -> None:
        red_url = RED_NONE + url + RESET_ESCAPE
        cyan_200 = CYAN_NONE + "200" + RESET_ESCAPE
        message = [
            f"Couldn't fetch data from the",
            f"'{red_url}' url!",
            "",
            f"Probably a non {cyan_200} status code has been returned."
        ]
        super().__init__(message, 1)
