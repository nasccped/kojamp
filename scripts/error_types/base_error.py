from colors import apply, YELLOW_NONE

class BaseError:
    """
    All error classes in this program are derived from `BaseError`
    class.

    It shouldn't be directly instantiated. All 'instantiable' errors
    can be founded in `derived_errors` module. They have pre-build
    error messages!
    """

    def __init__(self, message: str | list[str], exit_stts: int = 1):
        """
        Used only by the `super().__init__()` function from child
        classes.
        """
        self.error_message = message
        self.exit_status   = exit_stts

    def print_content(self):
        """
        Turn the error message into 'printable' type (into `str` if
        `list[str]`), and then, prints it.
        """
        message = self.error_message
        if isinstance(message, list):
            message = "\n".join(message)
        title = self.__class__.__name__
        colored_title = apply(title, YELLOW_NONE)
        print(f"{colored_title}", end="\n\n")
        print(message, end="\n\n")

    @staticmethod
    def exit_with_status(status: int):
        import sys
        sys.exit(status)
