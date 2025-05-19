def program_begin():
    from colors import (WHITE_NONE, GREEN_NONE, RESET_ESCAPE)
    title = "Kojamp Publish Script"

    print()
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print(f"{GREEN_NONE}{title.center(70)}{RESET_ESCAPE}")
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print()

def program_errors():
    from colors import (WHITE_NONE, RED_NONE, RESET_ESCAPE)
    title = "Errors"

    print()
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print(f"{RED_NONE}{title.center(70)}{RESET_ESCAPE}")
    print(f"{WHITE_NONE}{'=' * 70}{RESET_ESCAPE}")
    print()
