import re
from error_types.base_error import BaseError
from error_types.derived_errors import FileNotFound
from error_types.derived_errors import UnfetchableDataFile
from models.program_version import ProgramVersion
from utils.file import read_file_else_none
from utils.regex import pattern_in_liststr_sentence, pattern_in_str_sentence

def get_package_version_from_toml(
    toml_file: list[str] | None
) -> str | None:
    """
    Unwrap `package["version"]` field from a given str
    (toml structure). Will return None if the given str is None or
    the target field doesn't exits.
    """
    if not toml_file:
        return None

    pat = r'(\[package\][\s\S]*?version\s*=\s*"[^"]*")'

    if not (result := pattern_in_liststr_sentence(pat, toml_file)):
        return None

    pat = r'version[\s\S]=[\s\S]"[^"]*"'

    if not (result := pattern_in_str_sentence(pat, result)):
        return None

    pat = r'"[^"]*"'
    if not (result := pattern_in_str_sentence(pat, result)):
        return None

    return result.replace("\"", "").replace("'", "")

class File:
    """
    Store File fields, such as:

    - name
    - content (if it exists)
    - version (when reading _.toml)
    """

    def __init__(self, file_name: str):
        content = read_file_else_none(file_name)
        version = get_package_version_from_toml(content)

        error = FileNotFound(file_name) \
            if content is None \
            else UnfetchableDataFile(file_name, "package[\"version\"]") \
                if version is None \
                else None

        self.file_name: str = file_name
        self.file_content: list[str] | None = content
        self.version: ProgramVersion | None = None \
            if version is None \
            else ProgramVersion(version)
        self.error = error

    def get_file_name(self) -> str:
        return self.file_name

    def unwrap_err(self) -> None | BaseError:
        return self.error
