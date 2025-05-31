from colors import WHITE_NONE, GREEN_NONE, RESET_ESCAPE, RED_NONE

def draw_title(title: str, escape: str):
    print()
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print(f"{escape}{title.center(70)}{RESET_ESCAPE}")
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print()

def init_banner():
    title = "Kojamp Publish Script"
    draw_title(title, GREEN_NONE)

def error_banner():
    title = "Errors"
    draw_title(title, RED_NONE)
