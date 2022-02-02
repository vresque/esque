![](binaries/brand/twitter_header_photo_2.png)

<p align="center">
    <h1 align="center">Esque</h1>
</p>

[![If_You_See_This_Tokei_Is_Down_Again](https://tokei.rs/b1/github/visionizer/esque)](https://github.com/visionizer/esque)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/visionizer/esque)
[![license](https://img.shields.io/github/license/visionizer/esque?style=for-the-badge)](https://github.com/visionizer/esque)

A modern microkernel - Uniting Past and Present


## Building (Using y.py)

### Esque.toml

Before even talking about the hand-written build-system, I need to mention
Esque.toml. This is a configuration file with a plethora of options available for customization. Setting this up took a lot of time, which is why it is now the standard for building the Esque OS.


### Building
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

As mentioned above, it's main inspiration is Minix, when it comes
to kernel-design, but, when for the userspace-design, Windows is used.

I recommend that one read our [Syscall Documentation](Documentation/syscall.md)


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