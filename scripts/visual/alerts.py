from colors import CYAN_NONE, RESET_ESCAPE, RED_NONE, GREEN_NONE

def waiting_alert():
    cyan_files = CYAN_NONE + "files" + RESET_ESCAPE
    cyan_urls = CYAN_NONE + "urls" + RESET_ESCAPE
    take_a_while = RED_NONE + "take a while" + RESET_ESCAPE

    print()
    print(f"Fetching data from {cyan_files} and {cyan_urls}!")
    print(f"This can {take_a_while}...")
    print()

def local_tag_should_be_greater_than_remote(
    local_tag: str, remote_tag: str
):
    green_local_tag = GREEN_NONE + local_tag + RESET_ESCAPE
    green_remote_tag = GREEN_NONE + remote_tag + RESET_ESCAPE
    red_greater = RED_NONE + "greater" + RESET_ESCAPE
    print(f"The local git tag is {green_local_tag} but the")
    print(f"remote is {green_remote_tag}. To proceed, the local tag")
    print(f"should be {red_greater}!")

def local_tag_conflict(git_tag: str, file_version: str):
    green_local_tag = GREEN_NONE + git_tag + RESET_ESCAPE
    green_file_version = GREEN_NONE + file_version + RESET_ESCAPE
    red_same = RED_NONE + "same" + RESET_ESCAPE
    file_version = GREEN_NONE + "file[version]" + RESET_ESCAPE
    print(f"The local git tag is {green_local_tag} but the")
    print(f"file version is {green_file_version}. To proceed, the")
    print(f"local git tag should be the {red_same} as the")
    print(f"{file_version} field!")
