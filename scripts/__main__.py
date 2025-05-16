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
