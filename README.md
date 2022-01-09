# Esque
A modern microkernel kernel with ELF Executables - Uniting Past and Present

## Building (Using y.py)

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


## Philosophy
Esque is a Windows-NT-esque and Minix-esque operating system.
It is a micro-kernel featuring a Windows-like userspace.

I recommend that one read our [Syscall Documentation](Documentation/syscall.md)

## The InitRamFs
In the InitRamFs, as of right now, no directories are supported.
You can create a new InitRamFs simply by putting files into the `initramfs/` subdirectory.
Then, using `./y.py initramfs` the finished initramfs is going to be found in `build/initramfs.tar`. The bootloader expects this file to be found on the root partition.

All files ending with `.sys` will then be loaded by the InitRamFs. It is expected that one of said `.sys` files loads the FileSystem.

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
- [x] Memory Allocation
- [x] Support the `alloc` crate
- [ ] Remapping the Page Table
- [ ] Syscalls
- [ ] Executables
- [x] Heap
- [ ] Load Kernel Modules
- [x] Load the InitRamFs
- [x] Finish the Ext2 driver
- [ ] Release Milestone 1  