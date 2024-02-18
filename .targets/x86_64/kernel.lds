OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

_KERNEL_OFFSET = 2M;
SECTIONS
{
	. = 2M;
	_KERNEL_START = .;

	.text : ALIGN(0x1000)
	{
		*(.text)
	}
	.data : ALIGN(0x1000)
	{
		*(.data)
	}
	.rodata : ALIGN(0x1000)
	{
		*(.rodata)
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