import util
import subprocess
import json_handler

def run_cargo_command_in_workspace(cwd, command, args, rerun=True):
    util.info(f"Executing cargo command 'cargo {command}' with flags '{args}' in directory '{cwd}'")
    # First Pass (With Output)
    if isinstance(args, str):
        list = args.split(" ")
        if list[0] == "":
            list.pop(0)
        if list[len(list) - 1] == "":
            list.pop(len(list) - 1)
    else:
        list = args

    code, _, _ = util.run(
        ["cargo", command, *list],
                cwd=cwd)

    if code != 0:
        # No Error
        return None
    
    util.success(f"Running second pass of cargo command 'cargo {command}' as it successfully returned with error code {code}")
    
    if rerun:
        _, stdout, _ = util.run(
            ["cargo", command, *list, "--message-format=json"],
            stdout=subprocess.PIPE,
            # Suppress StdErr
            stderr=subprocess.DEVNULL,
            cwd=cwd)
        return json_handler.find_executables_in_cargo_json(stdout)