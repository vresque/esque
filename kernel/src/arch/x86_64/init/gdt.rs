use crate::{arch::gdt::*, arch::segment::*};
use bks::Handover;

pub fn init_gdt(_: &mut Handover) {
    let limit = (core::mem::size_of::<GlobalDescriptorTable>() - 1)
        .try_into()
        .expect("GDT is way too large");
    let offset = GLOBAL_GDT.as_ptr();
    let mut desc = &mut GDTDescriptor::new(limit, offset) as *mut _;
    unsafe {
        upload_gdt(desc);
        upload_to_dx(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_ds(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_es(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_fs(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_gs(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_ss(Segment::new(Ring::Ring0, GdtEntryType::KernelData)); // 0x10
        upload_to_cs(Segment::new(Ring::Ring0, GdtEntryType::KernelCode)); // 0x08                                                  //upload_to_cs(Segment::raw(0x08)); // 0x08
        loop {}
    };
    loop {}
}
