// Known functions: syscall_dispatcher

.global syscall_handler
syscall_handler:
    call syscall_dispatcher
    sysretq