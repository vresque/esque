# Syscalls

## SYS_HAL_* vs SYS_{!HAL}*

Within the kernel, there are two different classes of
syscalls - The Hardware Abstraction Layer Syscalls (SYS_HAL_*)
and the SYS_* calls. The SYS_HAL_* calls are only available to privileged applications running in system space.


SYS_* syscalls are available for all applications and their numeric identifiers start at 0. The SYS_NUM_MAX is currently defined as 512


SYS_HAL_* syscalls are only available to drivers their numeric identifiers start at 512 and run up to 1024 (SYS_HAL_NUM_MAX is defined as SYS_NUM_MAX * 2)

Should said limits ever be exceeded (for whatever reason), then their numbers will continue this trend with the consecutive power of two being the next limit.

| Type        	| From 	| To   	|
|-------------	|------	|------	|
| SYS_*       	| 0    	| 512  	|
| SYS_HAL_*   	| 512  	| 1024 	|
| (SYS_*)     	| 1024 	| 2048 	|
| (SYS_HAL_*) 	| 2048 	| 4096 	|

## How to call a syscall

The system calls follow the linux kernel conventions
```
rax - The system call number + (After the system call) the return value (Usually 1 - 255, signifying an ERRNO)
rdi - The Process-Pointer
rsi - The Message Pointer
```

## SYS_ABORT : 1

Shuts the system down