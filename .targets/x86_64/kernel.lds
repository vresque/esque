OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

_KERNEL_OFFSET = 2M;
SECTIONS
{
	. = 2M;
	_KERNEL_START = .;

    .text : ALIGN(0x1000)
	{
		__text_start = .;
		*(.text .text.*)
		. = ALIGN(4096);
		__text_end = .;
	}
	.rodata : ALIGN(0x1000)
	{
		__rodata_start = .;
		*(.rodata .rodata.*)
		. = ALIGN(4096);
		__rodata_end = .;
	}

	.tdata : ALIGN(0x1000)
	{
		__tdata_start = .;
		KEEP(*(.tdata*))
		KEEP(*(.tbss*))
		__tdata_end = ALIGN(8);
	}

	.data : ALIGN(0x1000)
	{
		__data_start = .;
		*(.data .data.*)
		. = ALIGN(4096);
		__data_end = .;
	}


	.bss :
	{
		*(COMMON)
		__bss_start = .;
		*(.bss .bss.*)
		. = ALIGN(4096);
		__bss_end = .;
	}
	_KERNEL_END = .;

	/DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}