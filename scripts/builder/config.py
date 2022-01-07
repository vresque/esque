import util
import os
from typing import *

VERSION: str = ""
NAME: str = ""
MODE: str = ""
FEATURES: List[str] = []
MODULES: List[str] = []
NO_MODULES: bool = []
DOCUMENTATION: bool = False
ARCH: str = "x86_64"
CUSTOM_INITRAMFS: str = None



try:
    import toml
except ImportError:
    util.error("The 'toml' module was not found. Please install it")
    exit(1)

import toml

def adjust_config_values_based_on_parser(arguments):
    global VERSION
    global NAME
    global MODE
    global FEATURES
    global MODULES
    global NO_MODULES
    global DOCUMENTATION
    global ARCH
    global CUSTOM_INITRAMFS

    MODE = "release" if arguments.release else "debug"
    DOCUMENTATION = arguments.documentation and DOCUMENTATION
    FEATURES += arguments.features
    ARCH = ARCH if arguments.arch == "" else arguments.arch
    MODULES += arguments.modules
    NO_MODULES = arguments.no_modules and NO_MODULES
    CUSTOM_INITRAMFS = arguments.custom_initramfs if arguments.custom_initramfs is not None else CUSTOM_INITRAMFS


    pass

def parse_config():
    global VERSION
    global NAME
    global MODE
    global FEATURES
    global MODULES
    global NO_MODULES
    global DOCUMENTATION
    global ARCH
    global CUSTOM_INITRAMFS

    this_dir = os.path.dirname(os.path.abspath(__file__))
    toml_path = os.path.join(this_dir, "..", "..", "Esque.toml")
    with open(toml_path, "r") as file:
        cfg = toml.load(file)
        VERSION = cfg["package"]["version"]
        NAME = cfg["package"]["name"]
        FEATURES = cfg["package"]["features"]
        MODULES = cfg["modules"]["modules"]
        NO_MODULES = cfg["modules"]["no-modules"]
        MODE = cfg["package"]["mode"]
        DOCUMENTATION = cfg["package"]["documentation"]
        CUSTOM_INITRAMFS = cfg["package"]["initramfs"] if cfg["package"]["initramfs"] != "default" else None