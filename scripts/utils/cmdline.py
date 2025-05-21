import subprocess

def command_is_ok(
    command: str, args: list[str] | None = None
) -> bool:
    full_cmd = [command]
    if isinstance(args, list):
        full_cmd.extend(args)

    return subprocess.call(
        full_cmd,
        stdout = subprocess.DEVNULL,
        stderr = subprocess.DEVNULL
    ) == 0

def get_command_output(
    command: str, args: list[str] | None = None
) -> list[str]:
    full_cmd = [command]
    if isinstance(args, list):
        full_cmd.extend(args)

    result = subprocess.run(
        full_cmd, stdout=subprocess.PIPE, text=True
    )
    result = result.stdout.strip()

    return [row for row in result.split("\n")]

def run_command(command: str, args: list[str] | None = None):
    full_cmd = [command]
    if isinstance(args, list):
        full_cmd.extend(args)

    subprocess.run(full_cmd)
