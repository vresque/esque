ARCH ?= x86_64
MEMLIM ?= 512M

OUTDIR = build
BINDIR = binaries
FINAL = Esque.img
FPATH = $(OUTDIR)/$(FINAL)
MODE ?= debug

QEMU = qemu-system-$(ARCH)
QEMUFLAGS = \
	-drive file=$(FPATH),format=raw \
	-m $(MEMLIM) \
	-enable-kvm \
	-machine q35,accel=kvm:tcg \
	-cpu host \
	-drive if=pflash,format=raw,unit=0,file=$(BINDIR)/OVMF/OVMF_CODE.fd,readonly=on \
	-drive if=pflash,format=raw,unit=1,file=$(BINDIR)/OVMF/OVMF_VARS.fd \
	-net none \
	-serial stdio

all: kernel boot mkimg run

build: kernel boot mkimg

clean:
	rm -rf build || true
	rm -rf target || true
	mkdir build || true


.PHONY: kernel
kernel:
	$(MAKE) -C kernel build ARCH=$(ARCH)
	cp target/kernel/$(MODE)/kernel $(OUTDIR)/esque

.PHONY: boot
boot:
	$(MAKE) -C boot build ARCH=$(ARCH)
	cp target/boot/$(MODE)/boot.efi $(OUTDIR)/BOOTX64.EFI

mkimg:
	dd if=/dev/zero of=$(FPATH) bs=512 count=93750
	mkfs.vfat -F 32 $(FPATH)
	mmd -i $(FPATH) ::/EFI
	mmd -i $(FPATH) ::/EFI/BOOT
	mcopy -i $(FPATH) $(OUTDIR)/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(FPATH) $(OUTDIR)/esque ::
	mcopy -i $(FPATH) $(BINDIR)/efi-shell/startup.nsh ::

run:
	$(QEMU) $(QEMUFLAGS)