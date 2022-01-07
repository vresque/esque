import subprocess

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

def exec(progname, **argv):
    output = subprocess.run(progname, **argv)
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
    print(Colors.HEADER + Colors.WARNING + Colors.UNDERLINE + Colors.INFO + "warning:" + Colors.ENDC + Colors.WARNING + " ", end="")
    print(fmt.format(**kwargs), end="")
    print(Colors.ENDC)