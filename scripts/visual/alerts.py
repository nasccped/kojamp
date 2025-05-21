from colors import \
    apply,         \
    CYAN_NONE,     \
    RED_NONE,      \
    GREEN_NONE,    \
    YELLOW_NONE,   \
    WHITE_BLUE
from models.project import Project

def waiting_alert():
    cyan_files = apply("files", CYAN_NONE)
    cyan_urls = apply("urls", CYAN_NONE)
    take_a_while = apply("take a while", RED_NONE)

    print()
    print(f"Fetching data from {cyan_files} and {cyan_urls}!")
    print(f"This can {take_a_while}...")
    print()

def local_tag_should_be_greater_than_remote(project: Project):
    green_local_tag = apply(str(project.local.latest), GREEN_NONE)
    green_remote_tag = apply(str(project.remote.latest), GREEN_NONE)
    red_greater = apply("greater", RED_NONE)
    print(f"The local git tag is {green_local_tag} but the")
    print(f"remote is {green_remote_tag}. To proceed, the local tag")
    print(f"should be {red_greater}!")

def local_tag_conflict(project: Project):
    green_local_tag = apply(str(project.local.latest), GREEN_NONE)
    green_file_version = apply(str(project.file.version), GREEN_NONE)
    red_same = apply("same", RED_NONE)
    file_version = apply("file[version]", GREEN_NONE)
    print(f"The local git tag is {green_local_tag} but the")
    print(f"file version is {green_file_version}. To proceed, the")
    print(f"local git tag should be the {red_same} as the")
    print(f"{file_version} field!")

def local_version_should_be_greater_than_docker(project: Project):
    green_local_ver = apply(str(project.local.latest), GREEN_NONE)
    green_docker_ver = apply(str(project.dhub.latest), GREEN_NONE)
    print(f"The local version is {green_local_ver} but the")
    print(f"latest docker registry is {green_docker_ver}. To proceed, the")
    print(f"local version should be greater than the")
    print(f"latest registry field!")

def update_warning(project: Project):
    docker_reg = apply("docker registry", GREEN_NONE)
    crate_ver = apply("crate version", GREEN_NONE)
    git_tag = apply("git tag", GREEN_NONE)
    print(f"The {docker_reg}, {crate_ver} and {git_tag} is about")
    print("to be updated:")

    old_gtag = str(project.remote.latest)
    new_gtag = str(project.local.latest)
    old_cver = str(project.crate.latest)
    new_cver = str(project.file.version)
    old_dreg = str(project.dhub.latest)
    new_dreg = str(project.file.version)

    new_gtag = apply(new_gtag, YELLOW_NONE)
    new_cver = apply(new_cver, GREEN_NONE)
    new_dreg = apply(new_dreg, WHITE_BLUE)

    print(f"github   : {old_gtag} -> {new_gtag}")
    print(f"crates.io: {old_cver} -> {new_cver}")
    print(f"dockerhub: {old_dreg} -> {new_dreg}")

def command_running_alert(command: str, args: list[str] | None = None):
    full_cmd = [command]
    if isinstance(args, list):
        full_cmd.extend(args)

    full_call = " ".join([apply(p, GREEN_NONE) for p in full_cmd])
    print(f"Running `{full_call}`")
