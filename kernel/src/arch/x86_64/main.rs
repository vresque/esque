use crate::{arch::init, config::set_handover};
use bks::Handover;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    crate::init::config::init_config(&mut handover);
    init::gdt::init_gdt(&mut handover);
    crate::init::common::init_common(&mut handover);
    init::memory::init_initial_paging(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::pic::init_pic(&mut handover);
    init::pit::init_pit(&mut handover);
    init::memory::map_memory(&mut handover);
    crate::main();
    init::smp::init_smp(&mut handover);
    set_handover(handover);
    crate::main();
}
