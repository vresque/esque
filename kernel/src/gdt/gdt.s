// We use 64 Bits

upload_gdt:
    // RDI is the first argument according to the sysv64 abi
    lgdt [rdi]
    mov ax, 0x10

    // Data Segment Registers
    mov dx, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    // Code Segment Register using Far Jump
    // Pop into RDI (Will be the lgdt return val)
    pop rdi
    // Adress the Kernel Code
    mov rax, 0x08
    // Push 0x08
    push rax
    // Push Return Address
    push rdi

    // Far Return
    retfq

.global upload_gdt