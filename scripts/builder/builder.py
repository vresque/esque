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
from util import exec, info, error, success, warning
import json_handler
import config
import parser

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
    config.parse_config()

    success("Building Esque...")
    arguments = parser.parse_args()
    print(arguments)
    config.adjust_config_values_based_on_parser(arguments)
    print(config.CUSTOM_INITRAMFS)