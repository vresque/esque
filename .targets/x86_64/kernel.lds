OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

KERNEL_OFFSET = 2M;
_KERNEL_OFFSET = KERNEL_OFFSET;
SECTIONS
{
	. = KERNEL_OFFSET;
	_KERNEL_START = .;
	. = ALIGN(4K);
    .text :
	{
		__text_start = .;
		*(.text .text.*)
		. = ALIGN(4096);
		__text_end = .;
	}
	. = ALIGN(4K);
	.rodata :
	{
		__rodata_start = .;
		*(.rodata .rodata.*)
		. = ALIGN(4096);
		__rodata_end = .;
	}

	. = ALIGN(4K);
	.tdata :
	{
		__tdata_start = .;
		KEEP(*(.tdata*))
		KEEP(*(.tbss*))
		__tdata_end = ALIGN(8);
	}

	. = ALIGN(4K);
	.data :
	{
		__data_start = .;
		*(.data .data.*)
		. = ALIGN(4096);
		__data_end = .;
	}


	. = ALIGN(4K);
	.bss :
	{
		*(COMMON)
		__bss_start = .;
		*(.bss .bss.*)
		. = ALIGN(4096);
		__bss_end = .;
	}

	__end = .;
	_KERNEL_END = .;

	/DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}