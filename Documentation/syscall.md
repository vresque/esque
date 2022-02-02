# Syscalls

## How to call a syscall

The system calls follow the linux kernel conventions
```
rax - The system call number + (After the system call) the return value (Usually 1 - 255, signifying an ERRNO)
rdi - The Process-Pointer
rsi - The Message Pointer
```

## SYS_ABORT : 1

Shuts the system down