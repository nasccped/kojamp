from .core import \
    RESET_ESCAPE, \
    RED_NONE,     \
    GREEN_NONE,   \
    WHITE_NONE,   \
    CYAN_NONE,    \
    YELLOW_NONE

def apply(message, color_escape: str) -> str:
    return color_escape + message + RESET_ESCAPE
