class BaseError:
    """
    All error classes in this program are derived from `BaseError`
    class.

    It shouldn't be directly instantiated. All 'instantiable' errors
    can be founded in `derived_errors` module. They have pre-build
    error messages!
    """

    def __init__(self, message: str | list[str], exit_stts: int = 1) -> None:
        """
        Used only by the `super().__init__()` function from child
        classes.
        """

        self.error_message = message
        self.exit_status   = exit_stts

    def print_message(self) -> None:
        """
        Turn the error message into 'printable' type (into `str` if
        `list[str]`), and then, prints it.
        """
        message = self.error_message
        if isinstance(message, list):
            message = "\n".join(message)
        print(message)

    def exit_with_status(self) -> None:
        import sys
        sys.exit(self.exit_status)

    def raise_err(self):
        """
        Do the error termination job (print message + kill program).
        """
        self.print_message()
        self.exit_with_status()
