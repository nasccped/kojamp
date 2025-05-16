from error_types.base_error import BaseError
from error_types.derived_errors import FileNotFound

class Project:
    """
    Class to store the project related fields, such as:

    - versions:
        - version from `Cargo.toml`
        - version from `crates.io`
        - version from `latest docker registry`
        - version from `latest git tag`

    - urls:
        - docker url pushing

    - errors:
        - FileNotFound
        - ErrorUrl
        - ...
    """

    def __init__(self) -> None:
        from core import target_files
        from .file import File

        # load File object + error obj list
        file_obj = File(target_files.CARGO_TOML)
        error_list: list[tuple[bool, BaseError]] = [
            (not file_obj.it_exists(), FileNotFound.from_file(file_obj))
        ]

        # set them to Project fields
        self.file: File = file_obj
        self.error_queue = error_list

    # get the first error (if it exists)
    def unwrap_err(self) -> None | BaseError:
        for cond, obj in self.error_queue:
            if cond: return obj
        return None
