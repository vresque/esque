use super::single_instruction;

single_instruction!(clear_interrupts -> "cli");
single_instruction!(halt -> "hlt");
single_instruction!(reload_interrupt_flags -> "sti");
