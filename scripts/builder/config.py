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
STRIP: bool = True

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
MINIMAL_TOOLCHAIN: bool = False
APPS_CARGO_FLAGS: str = ""

BOOT_RUSTC_FLAGS: str = ""
KERNEL_RUSTC_FLAGS: str = ""
APPS_RUSTC_FLAGS: str = ""

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
    global MINIMAL_TOOLCHAIN
    global APPS_CARGO_FLAGS
    global BOOT_RUSTC_FLAGS
    global KERNEL_RUSTC_FLAGS
    global APPS_RUSTC_FLAGS

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

    MINIMAL_TOOLCHAIN = arguments.minimal_toolchain if arguments.minimal_toolchain != False else MINIMAL_TOOLCHAIN

    if KERNEL_FEATURES != [] and KERNEL_FEATURES != [""]:
        KERNEL_CARGO_FLAGS += " --features "
        this_str = ",".join(KERNEL_FEATURES)
        KERNEL_CARGO_FLAGS += this_str
        KERNEL_CARGO_FLAGS += " "

    if BOOT_FEATURES != [""] and BOOT_FEATRUES != [""]:
        BOOT_CARGO_FLAGS += "--features \""
        this_str = " ".join(BOOT_FEATURES)
        BOOT_CARGO_FLAGS += this_str
        BOOT_CARGO_FLAGS += "\" "


    if BOOT_MODE == "release":
        BOOT_CARGO_FLAGS += "--release "
    if KERNEL_MODE == "release":
        KERNEL_CARGO_FLAGS += "--release "

    for f in [KERNEL_CARGO_FLAGS, BOOT_CARGO_FLAGS, APPS_CARGO_FLAGS, KERNEL_RUSTC_FLAGS, BOOT_RUSTC_FLAGS, APPS_RUSTC_FLAGS]:
        if f == "" or f[-1] != ' ':
            # Empty ones require a space - filled ones do not
            f += " "
    
    for f in [KERNEL_RUSTC_FLAGS, BOOT_RUSTC_FLAGS, APPS_RUSTC_FLAGS]:
        if f == "" or f == " ":
            f = "none"

    # Add Target Files
    KERNEL_CARGO_FLAGS += f"--target ../.targets/{ARCH}/kernel.json "
    BOOT_CARGO_FLAGS += f"--target ../.targets/{ARCH}/boot.json "
    APPS_CARGO_FLAGS += f"--target ../../.targets/{ARCH}/apps.json"


    # They turn into list somewhere along the way FIXME
    KERNEL_CARGO_FLAGS = "".join(map(str, KERNEL_CARGO_FLAGS))
    BOOT_CARGO_FLAGS = "".join(map(str, BOOT_CARGO_FLAGS))
    APPS_CARGO_FLAGS = "".join(map(str, APPS_CARGO_FLAGS))
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
    global MINIMAL_TOOLCHAIN
    global APPS_CARGO_FLAGS
    global BOOT_RUSTC_FLAGS
    global KERNEL_RUSTC_FLAGS
    global APPS_RUSTC_FLAGS


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
        rustc_flags = cfg["package"]["rustc-flags"]
        # Kernel Options
        KERNEL_MODE = cfg["kernel"]["mode"] if cfg["kernel"]["mode"] != "mirror" else MODE
        this_str = " ".join(cfg["kernel"]["cargo-flags"])
        KERNEL_CARGO_FLAGS = this_str if this_str != "mirror" else copy.deepcopy(flags)
        rustc_string = " ".join(cfg["kernel"]["rustc-flags"])
        KERNEL_RUSTC_FLAGS = rustc_string if rustc_string != "mirror" else copy.deepcopy(rustc_flags)
        KERNEL_FEATURES = cfg["kernel"]["features"]

        # Application Options
        app_cargo_flags = " ".join(cfg["apps"]["cargo-flags"])
        APPS_CARGO_FLAGS = app_cargo_flags if app_cargo_flags != "mirror" else copy.deepcopy(flags)
        rustc_apps_string = " ".join(cfg["apps"]["rustc-flags"])
        BOOT_RUSTC_FLAGS = rustc_apps_string if rustc_apps_string != "mirror" else copy.deepcopy(rustc_apps_string)

        
        # Bootloader Options
        BOOT_MODE = cfg["boot"]["mode"] if cfg["boot"]["mode"] != "mirror" else MODE
        my_str = " ".join(cfg["boot"]["cargo-flags"])
        BOOT_CARGO_FLAGS = my_str if my_str != "mirror" else copy.deepcopy(flags)
        rustc_boot_string = " ".join(cfg["boot"]["rustc-flags"])
        BOOT_RUSTC_FLAGS = rustc_boot_string if rustc_boot_string != "mirror" else copy.deepcopy(rustc_flags)

        BOOT_FEATURES = cfg["boot"]["features"]
        STRIP = cfg["package"]["strip"]

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
        MINIMAL_TOOLCHAIN = cfg["package"]["minimal-toolchain"]