import util
import json

def find_executables_in_cargo_json(stdout):
    ret = []
    for line in stdout.splitlines():
        current_json = json.loads(line)
        executable = current_json["executable"] if "executable" in current_json else None

        if executable != None:
            # Is *not* None at this point
            ret.append(current_json["executable"])

    return ret