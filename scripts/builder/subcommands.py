from pathlib import Path
from scripts.builder.cargo import run_cargo_command_in_workspace
from util import *
import config
import shutil
import subprocess
import math
import cargo
import os
import pathlib
import cliparser

QEMU = f"qemu-system-{config.ARCH}"


def build() -> int:
    code = initramfs()
    code = ~format() & ~code
    code = ~build_kernel() & ~code
    code = ~build_boot() & ~code
    code = ~strip() & ~code
    code = ~image() & ~code

    if config.DOCUMENTATION:
        code = ~build_docs() & ~code

    if config.SHOULD_RUN:
        code = ~run_qemu() & ~code

    if code == -1 or code == 0:
        return 0
    else:
        return 1

def clean() -> int:
    # Any Errors (Such as the directory already being gone, are ignored)
    try:
        shutil.rmtree("build")
        shutil.rmtree("target")
        os.mkdir("build")
        return 0
    except:
        return 0

def clippy() -> int:
    return 0

def all() -> int:
    return 0

def format() -> int:
    if not config.MINIMAL_TOOLCHAIN:
        if cargo.run_cargo_command_in_workspace(".", "fmt", [], "none") == None:
            return 1
    return 0

def run_qemu(in_background=False, return_exit_code=False, exit_on_error=True) -> int:
    # If never-run is set, ignore
    if config.NEVER_RUN:
        error("Not running due to either one of the following conditions being true")
        error("\t 1) The --never-run flag was passed to the tool")
        error("\t 2) In the given config, never-run was set to true")
        error("Here is what you can do:")
        error("\t 1) Pass the --disable-never-run flag to the tool")
        error("\t 2) Remove the command line flag that you passed to the tool")
        error("\t 3) Change the line in the configuration file")
        return 1

    QEMU_FLAGS = [
        f"-drive file={config.OUT_IMG},format=raw" if config.OUT_IMG != "" else "-drive file=build/esque-m,format=raw",
        f"-m {config.MEMLIM}",
        "-enable-kvm" if config.QEMU_KVM else f"-cpu {config.QEMU_CPU}",
        f"-machine {config.QEMU_MACHINE},accel=kvm:tcg",
        "-drive if=pflash,format=raw,unit=0,file=binaries/OVMF/OVMF_CODE.fd,readonly=on",
        "-drive if=pflash,format=raw,unit=1,file=binaries/OVMF/OVMF_VARS.fd",
        "-d", "int,cpu_reset",
        "-D" if config.QEMU_SHOULD_LOG else "",
        config.QEMU_LOGFILE if config.QEMU_SHOULD_LOG else "",
        "-no-shutdown", "-no-reboot",
        f"-smp {config.QEMU_SMP}",
        "-device isa-debug-exit,iobase=0xf4,iosize=0x04",
        *config.QEMU_OPTS,
    ]

    # It doesnt accept it as a list ???
    arr = [QEMU, *QEMU_FLAGS]
    one = " ".join(arr)

    if in_background:
        run_in_bg(one)
        info("Running QEMU in background")
        return 0
    else:
        code = run(one, exit_on_error=exit_on_error)[0]
        if return_exit_code:
            return code
        return 0
    return 0
    
def build_kernel() -> int:
    cargo.run_cargo_command_in_workspace("kernel", "build", config.KERNEL_CARGO_FLAGS, config.KERNEL_RUSTC_FLAGS)
    shutil.copy(f"target/kernel/{config.KERNEL_MODE}/kernel", "build/esque")
    return 0

def build_boot() -> int:
    cargo.run_cargo_command_in_workspace("boot", "build", config.BOOT_CARGO_FLAGS, config.BOOT_RUSTC_FLAGS)
    shutil.copy(f"target/boot/{config.BOOT_MODE}/boot.efi", "build/BOOTX64.EFI")
    return 0

def debug() -> int:
    config.STRIP = True
    build()
    config.QEMU_OPTS += ["-s", "-S"]
    run_qemu(True)
    run(["gdb", "--command=debug.gdb"])
    

def strip() -> int:
    run(["objcopy", "--only-keep-debug", "build/esque", "build/esque.sym"])
    if config.STRIP:
        run(["objcopy", "--strip-debug", "build/esque"])
        info("Striping binaries...")
        run(["strip", "build/esque"])
        run(["strip", "build/BOOTX64.EFI"])
        success("Successfully stripped all binaries")
    return 0

def image():
    info("Making Image...")
    run(["dd", "if=/dev/zero", f"of={config.OUT_IMG}", "bs=512", "count=93750"])
    run(["mkfs.vfat", "-F32", f"{config.OUT_IMG}"])
    run(["mmd", "-i", f"{config.OUT_IMG}", "::/EFI"])
    run(["mmd", "-i", f"{config.OUT_IMG}", "::/EFI/BOOT"])
    run(["mcopy", "-i", f"{config.OUT_IMG}", "build/BOOTX64.EFI", "::/EFI/BOOT"])
    run(["mcopy", "-i", f"{config.OUT_IMG}", "build/esque", "::"])
    run(["mcopy", "-i", f"{config.OUT_IMG}", "binaries/font/font.psf", "::"])
    run(["mcopy", "-i", f"{config.OUT_IMG}", "binaries/efi-shell/startup.nsh", "::"])
    run(["mcopy", "-i", f"{config.OUT_IMG}", "build/initramfs.tar", "::"])
    success(f"Successfully made image ({config.OUT_IMG})")
    return 0

def initramfs():
    info("Creating InitRamFs")
    run(["tar", "-cvf", "build/initramfs.tar", f"{config.CUSTOM_INITRAMFS}"])
    success("Successfully created the InitRamFs (build/initramfs.tar")
    return 0

def cloc():
    run(["bash", "scripts/cloc.sh"])
    return 0

def count_unsafe():
    run(["bash", "scripts/unsafe-counter.sh"])
    return 0

def build_docs():
    cargo.run_cargo_command_in_workspace(".", "doc", f"--no-deps", "none")


    with open("target/doc/index.html", "w+") as f:
        f.write('<meta http-equiv="refresh" content="0; url=kernel">')
    
    shutil.copytree("target/doc", "www/", dirs_exist_ok=True)
    return 0

def setup():
    try:
        os.mkdir("build")
        os.mkdir("build/www")
        return 0
    except:
        pass
    return 0

def build_testing_kernel() -> int:
    testing_kernel_bin = cargo.run_cargo_command_in_workspace("kernel", "test", "--no-run " + config.KERNEL_CARGO_FLAGS, config.KERNEL_RUSTC_FLAGS)
    shutil.copy(testing_kernel_bin[0], "build/esque")
    return 0


def test_inner(should_run=False) -> int:
    code = initramfs()
    code = ~format() & ~code
    code = ~build_testing_kernel() & ~code
    code = ~build_boot() & ~code
    code = ~strip() & ~code
    code = ~image() & ~code

    if config.DOCUMENTATION:
        code = ~build_docs() & ~code

    if should_run == True:
        info("Running QEMU")
        code = ~run_qemu() & ~code

    if code == -1 or code == 0:
        return 0
    else:
        return 1

    return 0

def test() -> int:
    return test_inner(True)

def apps() -> int:
    path = pathlib.Path("apps")
    apps = [f for f in path.iterdir() if f.is_dir()]
    for app in apps:
        appname: str = apps.__str__().replace("\\", "/").split("/")[-1].replace("')]", "")
        binpath: str = f"target/apps/{config.MODE}/" + appname
        run_cargo_command_in_workspace(app, "build", config.APPS_CARGO_FLAGS, config.APPS_RUSTC_FLAGS)
        
        if os.path.exists(app / ".initramfs"):
            with open(app / ".initramfs") as initramfs:
                exported_name = initramfs.readline()
                shutil.copy2(binpath, "initramfs/" + exported_name)
        else:
            shutil.copy2(binpath, "build/" + appname)

    return 0

def new_app() -> int:
    name = input("What is the name of the app? ")
    success(f"Creating app {name}")
    path_of_app = pathlib.Path("apps/" + name)
    
    try:
        os.mkdir(path_of_app.__str__())
    except:
        error(f"App {name} already exists.")
        opt = input("Deleate? (y/n;Y/N;Yes/No;yes/no) ").lower()
        # I would love a match statement here, but it is not fair to expect Python 3.10+
        if opt == "y" or opt == "yes":
            shutil.rmtree(path_of_app)
            os.mkdir(path_of_app)
            pass
        else:
            return 1

    run_cargo_command_in_workspace(path_of_app, "init", ["--bin"], "none", rerun=False)
    with open(path_of_app / "Cargo.toml", "a") as cargo:
        cargo.write("esque = { path = \"../../libesque/rust/esque\" }")
    
    with open(path_of_app / "src" / "main.rs", "w") as main:
        main.write("#![no_std]\n")
        main.write("#![no_main]\n")
        main.write("extern crate esque;\n")
        main.write("\n")
        main.write("\n")
        main.write("#[no_mangle]\n")
        main.write("pub fn main() {\n")
        main.write("    // Your code goes here\n")
        main.write("}\n")

    os.mkdir(path_of_app / ".cargo")
    with open(path_of_app / ".cargo" / "config.toml", "w+") as cfg:
        cfg.write("[unstable]\n")
        cfg.write("build-std-features = [\"compiler-builtins-mem\"]\n")
        cfg.write("build-std = [\"core\", \"compiler_builtins\", \"alloc\"]\n")



    opt = input("Should this app be in the InitRamFs? (y/n;Y/N;Yes/No;yes/no) ").lower()
    # I would love a match statement here, but it is not fair to expect Python 3.10+
    if opt == "y" or opt == "yes":
        f = open(path_of_app / ".initramfs", "w")
        inp = input(f"Under what name shall this app be exported into the InitRamFs? (Default: '{name}'. Leave blank for default.)")
        exported_name = inp if inp != "" else name
        f.write(exported_name)
        f.close()
        os.mkdir(path_of_app)
        pass
    else:
        return 1
    

    return 0

def run_ci() -> int:
    # Runs the CI
    
    # This is a bit "hacky". but it would be hard to accomplish otherwise
    config.KERNEL_CARGO_FLAGS = []
    arguments = cliparser.parse_args()
    # Force the recalculation of globals
    config.parse_config(arguments.config)
    config.KERNEL_FEATURES += ["harsh-tests"]
    config.adjust_config_values_based_on_parser(arguments)
    
    
    test_inner(False)
    success("Running Qemu...")
    outcode = run_qemu(return_exit_code=True, exit_on_error=False)
    info(f"QEMU returned exit code {outcode}")
    # QEMU cannot exit with 0 (If the user is in control)
    # QEMU calculates the exit code like this:
    # ((code << 1) | 1)
    # AKA: code * 2 + 1
    if outcode != 0x42 * 2 + 1:
        def translate_code(code):
            # Match Statements are not supported widely yet
            if code == 0x43:
                return "Total Failure (>50% of all tests failed)", 1
            elif code == 0x44:
                return "Mixed Results (<50% of all tests failed)", 2
            else:
                return "Invalid Exit Code", 254

        error(f"CI Failed with code {hex(math.ceil(outcode / 2 - 1))} (= {translate_code(math.ceil(outcode / 2 - 1))[0]})")
        return translate_code(outcode * 2 + 1)[1]
        
    success("CI succeeded completely (100% of all tests succeeded)")
    return 0
