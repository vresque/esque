from util import *
import config
import shutil
import subprocess
import cargo

QEMU = f"qemu-system-{config.ARCH}"
QEMU_FLAGS = [
    "-drive file=build/esque-vm.img,format=raw",
    f"-m {config.MEMLIM}",
    "-enable-kvm" if config.QEMU_KVM else f"-cpu {config.QEMU_CPU}",
    f"-machine {config.QEMU_MACHINE},accel=kvm:tcg",
    "-drive if=pflash,format=raw,unit=0,file=binaries/OVMF/OVMF_CODE.fd,readonly=on ",
    "-drive", "if=pflash,format=raw,unit=1,file=binaries/OVMF/OVMF_VARS.fd",
    "-net", "none",
    "-d", "int,cpu_reset,guest_errors,page,strace",
    "-D" if config.QEMU_SHOULD_LOG else "",
    config.QEMU_LOGFILE if config.QEMU_SHOULD_LOG else "",
    "-no-shutdown", "-no-reboot",
    f"-smp {config.QEMU_SMP}",
    *config.QEMU_OPTS,
]

def build() -> int:
    code = initramfs()
    code = ~format() & ~code
    code = ~build_kernel() & ~code
    code = ~build_boot() & ~code
    code = ~strip() & ~code
    code = ~image() & ~code
    if code == -1:
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
    if cargo.run_cargo_command_in_workspace(".", "fmt", []) == None:
        return 1
    return 0

def run_qemu() -> int:
    # It doesnt accept it as a list ???
    one_string: str = " ".join([QEMU, *QEMU_FLAGS])
    run(one_string)
def build_kernel() -> int:
    cargo.run_cargo_command_in_workspace("kernel", "build", config.KERNEL_CARGO_FLAGS)
    shutil.copy(f"target/kernel/{config.KERNEL_MODE}/kernel", "build/esque")
    return 0

def build_boot() -> int:
    cargo.run_cargo_command_in_workspace("boot", "build", config.BOOT_CARGO_FLAGS)
    shutil.copy(f"target/boot/{config.BOOT_MODE}/boot.efi", "build/BOOTX64.EFI")
    return 0

def strip() -> int:
    info("Striping binaries...")
    run(["strip", "build/esque"])
    run(["strip", "build/BOOTX64.EFI"])
    success("Successfully stripped all binaries")
    return 0

def image():
    info("Making Image...")
    run(["dd", "if=/dev/zero", "of=build/esque-vm.img", "bs=512", "count=93750"])
    run(["mkfs.vfat", "-F32", "build/esque-vm.img"])
    run(["mmd", "-i", "build/esque-vm.img", "::/EFI"])
    run(["mmd", "-i", "build/esque-vm.img", "::/EFI/BOOT"])
    run(["mcopy", "-i", "build/esque-vm.img", "build/BOOTX64.EFI", "::/EFI/BOOT"])
    run(["mcopy", "-i", "build/esque-vm.img", "build/esque", "::"])
    run(["mcopy", "-i", "build/esque-vm.img", "binaries/font/font.psf", "::"])
    run(["mcopy", "-i", "build/esque-vm.img", "binaries/efi-shell/startup.nsh", "::"])
    run(["mcopy", "-i", "build/esque-vm.img", "build/initramfs.tar", "::"])
    success("Successfully made image (build/esque-vm.img)")
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
