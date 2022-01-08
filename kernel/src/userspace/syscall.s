// %macro pop_all_registers 0
//     pop r15
//     pop r14
//     pop r13
//     pop r12
//     pop r11
//     pop r10
//     pop r9
//     pop r8
//     pop rbp
//     pop rdi
//     pop rsi
//     pop rdx
//     pop rcx
//     pop rbx
// %endmacro//
// %macro push_all_registers 0//
//     push rax
//     push rbx
//     push rcx
//     push rdx
//     push rsi
//     push rdi
//     push rbp
//     push r8
//     push r9
//     push r10
//     push r11
//     push r12
//     push r13
//     push r14
//     push r15//
// %endmacro//
//     .global __syscall
// __syscall:
//     swapgs ; Swap from User gs to kernel gs
//     mov [gs:0x8], rsp ; save current stack
//     mov rsp, [gs:0x0] ; Use the specified stack//
//     push qword 0x1b
//     push qword [gs:0x8]
//     push r11
//     push qword 0x23
//     push rcx//
//     cld
//     push_all_registers//
//     mov rdi, rsp
//     mov rbp, 0//
//     extern handle_syscall
//     call handle_syscall
//     pop_all_registers ; (Except Rax, return value)
//     mov rsp, [gs:0x8] ; Return to user stack//
//     swapgs 
//     o64 sysret