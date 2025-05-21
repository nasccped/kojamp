from colors import apply, CYAN_NONE, RED_NONE, GREEN_NONE

def waiting_alert():
    cyan_files = apply("files", CYAN_NONE)
    cyan_urls = apply("urls", CYAN_NONE)
    take_a_while = apply("take a while", RED_NONE)

    print()
    print(f"Fetching data from {cyan_files} and {cyan_urls}!")
    print(f"This can {take_a_while}...")
    print()

def local_tag_should_be_greater_than_remote(
    local_tag: str, remote_tag: str
):
    green_local_tag = apply(local_tag, GREEN_NONE)
    green_remote_tag = apply(remote_tag, GREEN_NONE)
    red_greater = apply("greater", RED_NONE)
    print(f"The local git tag is {green_local_tag} but the")
    print(f"remote is {green_remote_tag}. To proceed, the local tag")
    print(f"should be {red_greater}!")

def local_tag_conflict(git_tag: str, file_version: str):
    green_local_tag = apply(git_tag, GREEN_NONE)
    green_file_version = apply(file_version, GREEN_NONE)
    red_same = apply("same", RED_NONE)
    file_version = apply("file[version]", GREEN_NONE)
    print(f"The local git tag is {green_local_tag} but the")
    print(f"file version is {green_file_version}. To proceed, the")
    print(f"local git tag should be the {red_same} as the")
    print(f"{file_version} field!")
