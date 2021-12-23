use crate::gdt::*;
use bks::Handover;

pub fn init_gdt(handover: &mut Handover) {
    let gdt_size = core::mem::size_of::<GlobalDescriptorTable>() - 1;
    let gdt_offset = (&GLOBAL_GDT as *const GlobalDescriptorTable as *const u64) as u64;
    let mut gdt_desc = GDTDescriptor::new(gdt_size as u16, gdt_offset);

    unsafe { upload_gdt(&mut gdt_desc as *mut GDTDescriptor) };
}
