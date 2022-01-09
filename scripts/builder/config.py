import util
import os
from typing import *
import copy

VERSION: str = ""
NAME: str = ""
MODE: str = ""
MODULES: List[str] = []
NO_MODULES: bool = []
DOCUMENTATION: bool = False
ARCH: str = "x86_64"
CUSTOM_INITRAMFS: str = "initramfs"
KERNEL_CARGO_FLAGS: str = ""
BOOT_CARGO_FLAGS: str = ""
KERNEL_MODE: str = ""
BOOT_MODE: str = ""
BOOT_FEATRUES: List[str] = []
KERNEL_FEATURES: List[str] = []
MEMLIM: str = "512M"

QEMU_KVM = False
QEMU_CPU = "qemu64"
QEMU_LOGFILE = "qemu.log"
QEMU_MACHINE = "q35"
QEMU_OPTS = []
QEMU_SHOULD_LOG = True
QEMU_SMP = 1
SHOULD_RUN: bool = False
NEVER_RUN: bool = False
OUT_IMG: str = ""

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
    global MODULES
    global NO_MODULES
    global DOCUMENTATION
    global ARCH
    global CUSTOM_INITRAMFS
    global KERNEL_MODE
    global KERNEL_CARGO_FLAGS
    global BOOT_MODE
    global BOOT_CARGO_FLAGS
    global ARCH
    global BOOT_FEATURES
    global KERNEL_FEATURES
    global SHOULD_RUN
    global NEVER_RUN
    global OUT_IMG


    MODE = "release" if arguments.release else "debug"
    DOCUMENTATION = arguments.documentation
    ARCH = ARCH if arguments.arch == "" else arguments.arch
    MODULES += arguments.modules
    NO_MODULES = arguments.no_modules and NO_MODULES
    CUSTOM_INITRAMFS = arguments.custom_initramfs if arguments.custom_initramfs is not None else CUSTOM_INITRAMFS
    KERNEL_FEATURES += arguments.kernel_features
    BOOT_FEATURES += arguments.boot_features
    SHOULD_RUN = arguments.run
    NEVER_RUN = arguments.never_run
    NEVER_RUN = False if arguments.disable_never_run else NEVER_RUN

    OUT_IMG = arguments.outimage if arguments.outimage != "config" else OUT_IMG


    if KERNEL_FEATURES != [] and KERNEL_FEATURES != [""]:
        KERNEL_CARGO_FLAGS += " --features \""
        this_str = " ".join(KERNEL_FEATURES)
        KERNEL_CARGO_FLAGS += this_str
        KERNEL_CARGO_FLAGS += "\" "

    if BOOT_FEATURES != [""] and BOOT_FEATRUES != [""]:
        BOOT_CARGO_FLAGS += "--features \""
        this_str = " ".join(BOOT_FEATURES)
        BOOT_CARGO_FLAGS += this_str
        BOOT_CARGO_FLAGS += "\" "


    if BOOT_MODE == "release":
        BOOT_CARGO_FLAGS += "--release "
    if KERNEL_MODE == "release":
        KERNEL_CARGO_FLAGS += "--release "

    # Add Target Files
    KERNEL_CARGO_FLAGS += f"--target ../.targets/{ARCH}/kernel.json "
    BOOT_CARGO_FLAGS += f"--target ../.targets/{ARCH}/boot.json "


    # They turn into list somewhere along the way FIXME
    KERNEL_CARGO_FLAGS = "".join(map(str, KERNEL_CARGO_FLAGS))
    BOOT_CARGO_FLAGS = "".join(map(str, BOOT_CARGO_FLAGS))
    pass

def parse_config(config_path):
    global VERSION
    global NAME
    global MODE
    global MODULES
    global NO_MODULES
    global DOCUMENTATION
    global ARCH
    global CUSTOM_INITRAMFS
    global KERNEL_MODE
    global KERNEL_CARGO_FLAGS
    global BOOT_MODE
    global BOOT_CARGO_FLAGS
    global BOOT_FEATURES
    global ARCH
    global KERNEL_FEATURES
    global MEMLIM
    global QEMU_KVM
    global QEMU_CPU
    global QEMU_LOGFILE
    global QEMU_MACHINE
    global QEMU_OPTS
    global QEMU_SMP
    global QEMU_SHOULD_LOG
    global SHOULD_RUN
    global NEVER_RUN
    global OUT_IMG


    this_dir = os.path.dirname(os.path.abspath(__file__))
    toml_path = os.path.join(this_dir, "..", "..", config_path)
    with open(toml_path, "r") as file:
        cfg = toml.load(file)
        ARCH = cfg["package"]["arch"]
        VERSION = cfg["package"]["version"]
        NAME = cfg["package"]["name"]
        MODULES = cfg["modules"]["modules"]
        NO_MODULES = cfg["modules"]["no-modules"]
        MODE = cfg["package"]["mode"]
        DOCUMENTATION = cfg["package"]["documentation"]
        CUSTOM_INITRAMFS = cfg["package"]["initramfs"] if cfg["package"]["initramfs"] != "default" else "initramfs"
        flags = cfg["package"]["cargo-flags"]
        # Kernel Options
        KERNEL_MODE = cfg["kernel"]["mode"] if cfg["kernel"]["mode"] != "mirror" else MODE
        this_str = " ".join(cfg["kernel"]["cargo-flags"])
        KERNEL_CARGO_FLAGS = this_str if this_str != "mirror" else copy.deepcopy(flags)
        KERNEL_FEATURES = cfg["kernel"]["features"]
        # Bootloader Options
        BOOT_MODE = cfg["boot"]["mode"] if cfg["boot"]["mode"] != "mirror" else MODE
        my_str = " ".join(cfg["boot"]["cargo-flags"])
        BOOT_CARGO_FLAGS = my_str if my_str != "mirror" else copy.deepcopy(flags)
        BOOT_FEATURES = cfg["boot"]["features"]

        MEMLIM = cfg["qemu"]["memlim"]
        QEMU_KVM = cfg["qemu"]["enable-kvm"]
        QEMU_CPU = cfg["qemu"]["cpu"]
        QEMU_LOGFILE = cfg["qemu"]["logfile"]
        QEMU_SHOULD_LOG = cfg["qemu"]["should-log"]
        QEMU_SMP = cfg["qemu"]["smp"]
        QEMU_MACHINE = cfg["qemu"]["machine"]
        QEMU_OPTS = cfg["qemu"]["qemu-opts"]

        SHOULD_RUN = cfg["package"]["should-run"]
        NEVER_RUN = cfg["package"]["never-run"]
        OUT_IMG = cfg["package"]["out-image-path"]