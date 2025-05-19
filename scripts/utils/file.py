"""
Utils for file handling (reading, specifically)
"""

def read_file_else_none(file_name: str) -> list[str] | None:
    try:
        with open(file_name, "r") as f:
            result = f.readlines()
    except:
        return None
    return result


