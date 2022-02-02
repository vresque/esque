import subprocess
import os

WARN_COUNT: int = 0

def beautiful_exit(code):
    if code != 0:
        warn_str = ""
        if WARN_COUNT > 0:
            warn_str = f"{Colors.WARNING} and {WARN_COUNT} warnings{Colors.ENDC}" if WARN_COUNT != 1 else f"{Colors.WARNING} and {WARN_COUNT} warning{Colors.ENDC}"
        error(f"An error{warn_str}{Colors.FAIL} occured while building Esque")
        exit(code)
    else:
        warn_str = "without warnings"
        if WARN_COUNT > 0:
            warn_str = f"{Colors.WARNING}with {WARN_COUNT} warnings{Colors.ENDC}" if WARN_COUNT != 1 else f"{Colors.WARNING}with {WARN_COUNT} warning{Colors.ENDC}"

        success(f"Successfully built Esque {warn_str}{Colors.SUCCESS}")
        exit(code)
class Colors:
    HEADER = '\033[1m' + '\033[4m' + '\033[95m'
    INFO = '\033[94m'
    CYAN = '\033[96m'
    SUCCESS = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def run_in_bg(progarg, **kwargs):
    if isinstance(progarg, str):
        list = progarg.split(" ")
    else:
        list = progarg

    output = subprocess.Popen(list, **kwargs)

def run(progarg, **kwargs):
    if isinstance(progarg, str):
        print("arrb")
        list = progarg.split(" ")
    else:
        list = progarg

    output = subprocess.run(list, **kwargs)

    if output.returncode != 0:
        command = " ".join(map(str, list))
        if output.stdout != None:
            stdout_text = " ".join(map(str, output.stdout.splitlines()))
        else:
            stdout_text = "<No StdOut found>"
        
        if output.stderr != None:
            stderr_text = " ".join(map(str, output.stderr.splitlines()))
        else:
            stderr_text = "<No StdErr found>"
        error(f"The command '{Colors.UNDERLINE}{Colors.CYAN}{command}{Colors.ENDC}{Colors.FAIL}' executed in '{os.getcwd()}' failed with error code {output.returncode}.\nStdErr: {stderr_text}\nStdOut: {stdout_text}\nExiting...")
        beautiful_exit(output.returncode)
    return output.returncode, output.stdout, output.stderr,

def info(fmt, **kwargs):
    print(Colors.HEADER + "info:" + Colors.ENDC + Colors.INFO + " ", end="")
    print(fmt.format(**kwargs), end="")
    print(Colors.ENDC)

def error(fmt, **kwargs):
    print(Colors.HEADER + Colors.FAIL + Colors.UNDERLINE + Colors.INFO + "error:" + Colors.ENDC + Colors.FAIL + " ", end="")
    print(fmt.format(**kwargs), end="")
    print(Colors.ENDC)

def success(fmt, **kwargs):
    print(Colors.HEADER + Colors.SUCCESS + Colors.UNDERLINE + Colors.INFO + "success:" + Colors.ENDC + Colors.SUCCESS + " ", end="")
    print(fmt.format(**kwargs), end="")
    print(Colors.ENDC)

def warning(fmt, **kwargs):
    global WARN_COUNT
    WARN_COUNT += 1
    print(Colors.HEADER + Colors.WARNING + Colors.UNDERLINE + Colors.INFO + "warning:" + Colors.ENDC + Colors.WARNING + " ", end="")
    print(fmt.format(**kwargs), end="")
    print(Colors.ENDC)