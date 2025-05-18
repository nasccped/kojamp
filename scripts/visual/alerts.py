from colors import CYAN_NONE, RESET_ESCAPE, RED_NONE

def waiting_alert():
    cyan_files = CYAN_NONE + "files" + RESET_ESCAPE
    cyan_urls = CYAN_NONE + "urls" + RESET_ESCAPE
    take_a_while = RED_NONE + "take a while" + RESET_ESCAPE

    print(f"Fetching data from {cyan_files} and {cyan_urls}!")
    print(f"This can {take_a_while}...")


