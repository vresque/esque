    .global upload_pml4
upload_pml4:
    mov rax, rdi
    mov cr3, rax