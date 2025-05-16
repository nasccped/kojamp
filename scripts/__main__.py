"""
This is the publish script main file!

It should be ran in the repository root path.
You don't need to specify the file entry. Just use
`python <PATH_NAME> (scripts for this case)`
"""

def print_banner():
    from panel import banner
    print()
    banner.banner()
    print()

def main():
    from models import project

    project = project.Project()
    err = project.unwrap_err()
    if err is not None:
        err.raise_err()

if __name__ == "__main__":
    print_banner()
    main()
