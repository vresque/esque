![](binaries/brand/twitter_header_photo_2.png)

## Note: This project was deprecated in favour of [Clecx](https://github.com/visionizer/clecx)

<p align="center">
    <h1 align="center">Esque</h1>
</p>

[![If_You_See_This_Tokei_Is_Down_Again](https://tokei.rs/b1/github/visionizer/esque)](https://github.com/visionizer/esque)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/visionizer/esque)
[![license](https://img.shields.io/github/license/visionizer/esque?style=for-the-badge)](https://github.com/visionizer/esque)

A modern microkernel - Uniting Past and Present


## Building (Using y.py)

### Esque.toml

Before even talking about the hand-written build-system, I need to mentionvEsque.toml. This is a configuration file with a plethora of options available for customization. Setting this up took a lot of time, which is why it is now the standard for building the Esque OS.

### Dependencies (On Linux)
- `cargo`
- `rustc`
- `dd`
- `mtools` (mcopy, mmd, ...)
- `dosfstools` (mkfs.vfat)
- `python >= 3`
- `python.toml`
- `python.xbstrap`

#### Install
```sh
$ sudo apt install cargo rustc binutils mtools dosfstools python3 python3-pip; pip install --user xbstrap toml
```

### Building (On Linux)
`y.py` is an utility inspired by rustc's `x.py`. You can configure
the kernel using the `Esque.toml` file that may be found in the sysroot of this directory. This file offers many options, have a look at it before building.

You can build the project simply using
```
./y.py build
```
This system is very configurable. Simply type
```
./y.py --help
```
to see all options.

### Building (On Windows)

**First, you must enter `Esque.toml` and change `enable-kvm` to false.**

Building on Windows is not recommended. I am long-time linux user and the entire build process iis designed for me. As I have a work laptop which, unfortunately, comes preinstalled with a proprietary guard on my harddrive which prohibits the usage of any non-secureboot verified operating systems, I had to become proficient in "Windows Administrative Skills". This is also the reason why `winy` exists.


On Windows, only certain `y.py` commands may be executed in the same way as on Linux (Example: `./y.py` build runs `dd` to create an IMG file). Therefore, you are presented with two options

#### Option A: WSL + Native QEMU
This may be a preferred option for some. In this scenario, you run *all* commands **except** for `./y.py run` using WSL.

This requires all of the dependencies listed above in `Dependencies (On Linux)` section

##### Pros
- Fast

##### Cons
- You need to switch terminals each time you want to build and run it

#### Option B: Using ./winy.ps1
`winy.ps1` is a PowerShell script which decides what to run natively and what not. Usage is equal to `./y.py` e.g. `./winy run` runs the kernel and `./winy build` builds some parts of the kernel using WSL and others natively.

**_Attention_**
This requires you to have your ExecutionPolicy to be Bypass. You can temporarily change this by opening a command host with administrator privileges and typing
```ps1
Set-ExecutionPolicy Bypass
```

This requires all dependencies listed above *except* for cargo and rustc on WSL. It requires cargo, rustc, and a `tar` binary on on windows. Said dependencies may easily be installed using the rustup binary `rustup.rs`

###### Installing said dependencies
Run the following command on `WSL` (assumes Ubuntu):
```sh
$ sudo apt install binutils mtools dosfstools python3 python3-pip; pip install --user xbstrap toml
```

##### Pros
- More user-friendly

##### Cons
- WSL-Commands have a significant startup time

## Dependencies

An operating system should be close to being dependency free.
Unfortunatley, this system depends on a total of 2 crates:
```
bitflags
spin
```

Over 10+ of our own dependencies are maintained within the `crates/` subdirectory. These dependencies include a tar loader and much more.

- Bitflags is used to replace the C bitfields. This is a neccessary dependency. It is used in thousands of projects all over the world. I consider this a completely safe crate.
- Spin is a replacement for Rust's `std::sync::{Mutex,...}`. This is an incredibly helpful crate that is used in nearly all major osdev projects. This crate *might* be dropped in the future.

## Doesn't rust produce huge binaries?

While it may produce bigger binaries then, let's say, C, it still produces small ones after stripping. The current kernel is just ~300K in size, which is acceptable to me. The bootloader is about 270K big, due to its huge 'uefi' dependency.


## Philosophy
Esque is a Windows-NT-esque and Minix-esque operating system.
It is a micro-kernel featuring a Windows-like userspace.

As mentioned above, it's main inspirations are Linux and Windows, when it comes
to kernel-design, but, when for the userspace-design, Windows is used.

### Linux Compatibility

Due to the huge availability of software on linux, esque aims to be somewhat compatible with it. It achieves filesystem compatibility due to the use of a `fake-root`. There are two major. The *real root* and the *fake root*. An example of a *fake path* would be `/home/user/` or `/bin/*`. A *real root* path starts with the *device:PATH* scheme. Examples: `initramfs:/myfile`, `C:/Binaries/*`, `B:/BOOT/EFI/BOOTX64.EFI`, `C:/Users/User/`or `proc:/CpuInfo`.

Linux syscalls are located at their actual location (0, 1, 2, 3, 4...) while esque syscalls (Windows Like) are located at (SYS_NUM + 0x1000)

### Does everything run in Userspace?

Yes - and no. In Esque, there are three different *virtual* 'spaces' for applications. Only two of those are real.
There is
- Kernel Space (*Only the Kernel* - Has access to the full array of hardware)
- System Space (*Drivers* - *Runs in the Kernel Space of the CPU* Has access to IPC with the kernel and the HAL (Hal = The Hardware Abstraction Layer)
- User Space (*Applications* - Only has access to the system calls)


## Contributing

I understand that not many are willing to use their time on a kernel
such as this one. I will still gladly welcome any contribution, no matter how big or small. Please read the [Contributions File](CONTRIBUTING.md) and take a look at the files in the [Documentation Directory](Documentation)

## Microkernel? Why would you do that?

While the current narrative is that Monolithic kernel's are better than Microkernels, I believe that there may be a compromise between those two - The system described above. This system is loosely inspired by Windows NT's design - Which is also an entire system revolving around the Windows NT microkernel.

## The InitRamFs
In the InitRamFs, as of right now, no directories are supported.
You can create a new InitRamFs simply by putting files into the `initramfs/` subdirectory.
Then, using `./y.py initramfs` the finished initramfs is going to be found in `build/initramfs.tar`. The bootloader expects this file to be found on the root partition.

All files ending with `.system` will then be loaded by the InitRamFs. It is expected that one of said `.system` files loads the FileSystem.

## About Unsafe
While it is true that an operating system without unsafe code is impossible, I tried to limit it in here. At any point, 
```
./y.py count-unsafe
```
may be invoked which will display information about the unsafe-ness of the code.
At the time of writing, the following output is produced:
```
A total of 52 occurences have been found (1641 LOC, 0.* percent Percent)
```


## Roadmap
- [x] Own Bootloader
- [x] Font Loading
- [x] GDT
- [x] IDT
- [x] All Faults Handled
- [x] Port-IO
- [x] Fallback Drivers for Fallback-Shell (For debugging purposes)
- [x] Level 4 Paging
- [x] PS2 Keyboard
- [x] Keyboard Layout Crate
- [x] PS2 Mouse Support
- [x] Memory Allocation
- [x] Load TAR Files
- [x] Support the `alloc` crate
- [x] Remapping the Page Table
- [ ] Syscalls
- [ ] IPC-Messaging
- [ ] PCI Device Descriptors
- [ ] Executables
- [x] Heap
- [x] Load System Space Applications
- [x] Load the InitRamFs
- [ ] MultiTasking
- [ ] Finish the Ext2 driver
- [ ] Release Milestone 1  
- [ ] ACPI
- [ ] RSDP POINTER
- [ ] PCI Bus
- [ ] PCI Enumeration
- [ ] Full PIC Support
- [x] A lot more...
