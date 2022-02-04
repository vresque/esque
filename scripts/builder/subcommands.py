from util import *
import config
import shutil
import subprocess
import cargo
import os

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
        if cargo.run_cargo_command_in_workspace(".", "fmt", []) == None:
            return 1
    return 0

def run_qemu(in_background=False) -> int:
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
        run(one)
        return 0
    return 0
    
def build_kernel() -> int:
    cargo.run_cargo_command_in_workspace("kernel", "build", config.KERNEL_CARGO_FLAGS)
    shutil.copy(f"target/kernel/{config.KERNEL_MODE}/kernel", "build/esque")
    return 0

def build_boot() -> int:
    cargo.run_cargo_command_in_workspace("boot", "build", config.BOOT_CARGO_FLAGS)
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
    cargo.run_cargo_command_in_workspace(".", "doc", f"--no-deps")


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
    cargo.run_cargo_command_in_workspace("kernel", "test", "--no-run " + config.KERNEL_CARGO_FLAGS)
    shutil.copy(f"target/kernel/{config.KERNEL_MODE}/kernel", "build/esque")
    return 0


def test() -> int:
    code = initramfs()
    code = ~format() & ~code
    code = ~build_testing_kernel() & ~code
    code = ~build_boot() & ~code
    code = ~strip() & ~code
    code = ~image() & ~code

    if config.DOCUMENTATION:
        code = ~build_docs() & ~code

    code = ~run_qemu() & ~code

    if code == -1 or code == 0:
        return 0
    else:
        return 1

    return 0