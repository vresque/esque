#!/usr/bin/env python

# All Logic should be contained in scripts/builder/builder.py

import os
import sys

# Python3.* is required
if sys.version_info.major < 3:
    try:
        # python3 opens the Windows Store, sometimes even
        # on machines where python is installed (That happened on my old laptop)
        os.execvp("py", ["py", "-3"] + sys.argv)
        exit(0)
    except OSError:
        # No py installed, so python *should* be available
        try:
            os.execvp("python3", ["python3"] + sys.argv)
            exit(0)
        except OSError:
            print("Pre-Execution Error: No valid python3 interpreter found (Tried: `py` and `python3`")
            exit(1)

this_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.join(this_dir, "scripts", "builder"))

# Should be available now
import builder

if __name__ == "__main__":
    try:
        builder.main()
    except KeyboardInterrupt:
        exit(0)
else:
    print("Pre-Execution Error: Please do not import y.py. Either execute it, or load the builder file in scripts/builder")