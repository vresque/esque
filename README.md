# Esque
A modern microkernel kernel featuring executables in WebAssembly as well as ELF - Uniting Past and Present

## Philosophy

Esque is a Unix-esque operating system (Very funny). While the Everything Is A File-Philosophy is great,
I believe that it is time to extend this idea. In Esque, **Everything is an Object whose Pointer is stored as a File**. An example is illustrated below.

On boot, Esque calls the `initfs` binary from the initramfs which needs to return a `*mut FileSystem` (`FileSystem*` for the C-People). Then, the kernel uses this FileSystem to call the function `write` and write
the root filesystem onto itself. Any application may retrieve this FileSystem simply by using the `get_root_filesystem` syscall. This is the only syscall that deals with the FileSystem and should only be used during initialization. 

The following is a part of the layout of a `FileSystem`. Note that `$name_of_fn: fn($name_of_param: $type_of_param, ..) -> $return_type`
is equal to `$return_type(* $name_of_fn)($type_of_param $name_of_param, ..)` in C.
Additionally, the type `CString` is the same as `char*` in C.

```rs
#[repr(C)] // Has the same layout as a struct in C
pub struct FileSystem {
    open: fn(CString, CString, FileOpenAttribute)
    ... // And so on, for all of the required functions
}

## Screenshots
![A Blue Screen of Death (Kernel Panic) in Esque](binaries/screenshots/bsod.png)
A Blue Screen of Death in Esque (Kernel Panic)

![Debugging the Memory map](binaries/screenshots/mem-map.png)
A simple dump of the memory map


## Building

Run
```
make build
```
Then, run
```
make run
```
to run the OS using QEMU.

To clean, use
```
make clean
```

## About Unsafe
While it is true that an operating system without unsafe code is impossible,
I tried to limit it in here. At any point, 
```
make unsafe-counter
```
may be invoked which will display information about the unsafe-ness of the code.
At the time of writing, the following output is produced:
```
A total of 52 occurences have been found (1641 LOC, 0 Percent)
```


## Roadmap
- [x] Own Bootloader
- [x] Font Loading
- [x] GDT
- [x] IDT
- [ ] All Faults Handled
- [x] Port-IO
- [x] Fallback Drivers for Fallback-Shell (For debugging purposes)
- [x] Level 4 Paging
- [x] PS2 Keyboard
- [x] Keyboard Layout Crate
- [ ] Remapping the Page Table
- [ ] Fallback Kernel Shell
- [ ] Syscalls
- [ ] WebAssembly-Executables
- [ ] Heap
- [ ] Release Milestone 1  