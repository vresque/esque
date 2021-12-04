# Esque
A modern exokernel

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
of capabilities. Those permissions must be in the rocket-file of the application.

#### What is a Rocket File?
Rocket files (*.rck) are the applications of esque. **They are merely an extension of ELF, any ELF can be converted into a rocket file and vice-versa**.
This is what a Rocket-File looks like:
```
|************** + *************** + **************** + ******************* + ************* + **********|
| Rocket Magic  |  Static Header  |  Dynamic Header  |  Extensible Header  |  Memo Header  |  Full Elf |
|************** + *************** + **************** + ******************* + ************* + **********|
```
(More documentation to follow)

