OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

SECTIONS
{
	. = 2M;
    .text : ALIGN(4K)
	{
		*(.text .text.*)
	}
	.data : ALIGN(4K)
	{
		*(.data .data.*)
	}
	.rodata : ALIGN(4K)
	{
		*(.rodata .rodata.*)
	}
	.bss : ALIGN(4K)
	{
		*(COMMON)
		*(.bss .bss.*)
	}
}