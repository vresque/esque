# All Logic should be in here

import sys
import os
from typing import *
import argparse
import json
import shutil
import subprocess
import tarfile
import util
from util import beautiful_exit, run, info, error, success, warning
import json_handler
import config
import cliparser

try:
    import xbstrap
except:
    util.error("Pre-Builder Error: The xbstrap module was not installed. Please install it")
    exit(1)
# This file should not be executed directly
if __name__ == "__main__":
    print("Please do not directly execute builder.py. Execute y.py instead")
    exit(1)

def update_dependencies():
    """
    Updates the OVMF Dependency
    """
    pass


def main():
    arguments = cliparser.parse_args()
    config.parse_config(arguments.config)
    success("Building Esque...")
    config.adjust_config_values_based_on_parser(arguments)


    # Calling the right function
    code = cliparser.SUBCOMMANDS_TO_FN[arguments.subcommand]()
    beautiful_exit(code)