from .cmdline import run_command
from visual.alerts import command_running_alert

def cratesio_publish():
    cmd = "cargo"
    args = ["publish"]
    command_running_alert(cmd, args)
    run_command(cmd, args)

def github_publish():
    cmd = "git"
    args = ["push", "origin", "main"]
    command_running_alert(cmd, args)
    run_command(cmd, args)

    args = ["push", "origin", "--tags"]
    command_running_alert(cmd, args)
    run_command(cmd, args)

def docker_publish(new_version: str):
    cmd = "docker"
    args = ["build", "-t", "kojamp:scripted", "."]
    command_running_alert(cmd, args)
    run_command(cmd, args)

    new_registry = f"nasccped/kojamp:{new_version}"
    args = ["tag", "kojamp:scripted", new_registry]
    command_running_alert(cmd, args)
    run_command(cmd, args)

    args = ["push", new_registry]
    command_running_alert(cmd, args)
    run_command(cmd, args)

    new_registry = f"nasccped/kojamp:latest"
    args = ["tag", "kojamp:scripted", new_registry]
    command_running_alert(cmd, args)
    run_command(cmd, args)

    args = ["push", new_registry]
    command_running_alert(cmd, args)
    run_command(cmd, args)

