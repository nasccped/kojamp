from error_types.derived_errors import FileNotFound

class File:
    """
    Store File fields, such as:

    - name
    - content (if it exists)
    """

    def __init__(self, file_name: str):
        content = None
        error = None
        try:
            with open(file_name, "r") as f:
                content = f.readlines()
        except:
            error = FileNotFound(file_name)

        self.file_name: str = file_name
        self.file_content: list[str] | None = content
        self.error = error

    def get_file_name(self) -> str:
        return self.file_name

    def unwrap_err(self) -> None | FileNotFound:
        return self.error
