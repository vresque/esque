import util
import subprocess
import json_handler

def run_cargo_command_in_workspace(cwd, command, arguments):
    util.info("Executing cargo command '{}' in directory '{}'", command, cwd)
    code, stdout, _ = exec(
        ["cargo", command, *arguments, "--message-format=json"],
        stdout=subprocess.PIPE,
        # Suppress StdErr
        stderr=subprocess.DEVNULL,
        cwd=cwd)

    if code != 0:
        # No Error
        return None
    return json_handler.find_executables_in_cargo_json(stdout)