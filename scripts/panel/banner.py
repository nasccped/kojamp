from colors import (WHITE_NONE, GREEN_NONE, RESET_ESCAPE)

def banner():
    title = "Kojamp Publish Script"

    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print(f"{GREEN_NONE}{title.center(70)}{RESET_ESCAPE}")
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
