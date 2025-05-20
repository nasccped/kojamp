import subprocess

def command_is_ok(command: str, args: list[str] | None = None) -> bool:
    full_cmd = [command]
    if isinstance(args, list):
        full_cmd.extend(args)

    return subprocess.call(
        full_cmd,
        stdout = subprocess.DEVNULL,
        stderr = subprocess.DEVNULL
    ) == 0
