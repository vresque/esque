# Esque
A modern exokernel (In the future)

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

## What is an Exokernel

## Features

### Guards
Each aspect of the operating system is guarded by a fierce guard. For example, the framebuffer is guarded
by the FramebufferGuard.

### Capability-Based Applications
While Exokernels are based on the idea of trusting applications, this is not 100% neccessary. This is why all Applications have a set
of capabilities. Those permissions must be in the ELF file of the application.
If no capabilities are found in the ELF, a default set of capabilities is given
to the application. Those should suffice to run any *nix application.