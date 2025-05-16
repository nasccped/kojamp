class File:
    """
    Store File fields, such as:

    - name
    - content (if it exists)
    """

    def __init__(self, file_name: str):
        import os

        exists = file_name in os.listdir()
        content = None

        if exists:
            with open(file_name, "r") as f:
                content = f.readlines()

        self.file_name: str = file_name
        self.exists: bool = exists
        self.file_content: list[str] | None = content

    def it_exists(self) -> bool:
        return self.exists

    def get_file_name(self) -> str:
        return self.file_name
