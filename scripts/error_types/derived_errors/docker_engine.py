from colors import GREEN_NONE, RESET_ESCAPE, RED_NONE
from error_types.base_error import BaseError

class DockerEngineError(BaseError):

    def __init__(self):
        docker_cli_subcommands = GREEN_NONE \
            + "Docker cli subcommands" \
            + RESET_ESCAPE
        isnt_running = RED_NONE + "isn't running" + RESET_ESCAPE
        message = [
            "This error occurs when the program fail to call",
            f"{docker_cli_subcommands}. The Docker Engine",
            f"{isnt_running} (probably)."
        ]
        super().__init__(message, 1)

