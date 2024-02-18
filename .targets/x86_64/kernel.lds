OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

SECTIONS
{
	_KERNEL_START = .;
	.text : ALIGN(0x1000)
	{
		*(.text)
	}
	.rodata : ALIGN(0x1000)
	{
		*(.rodata)
	}
	.data : ALIGN(0x1000)
	{
		*(.data)
	}
	.bss : ALIGN(0x1000)
	{
		*(COMMON)
		*(.bss)
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