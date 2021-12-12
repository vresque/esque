use bks::Handover;

use crate::kinfo;
use crate::memory::bitmap::Bitmap;
use crate::{kprintln, memory::bitmap::BITMAP};

pub fn init_memory(handover: &mut Handover) {
    kprintln!("Initializing memory...");
    unsafe {
        kprintln!("Initializing Bitmap...");
        BITMAP.lock().write(Bitmap::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
        ));
        kprintln!("Total Memory: ");
        let total = BITMAP.lock().assume_init_mut().total_memory();
        kprintln!("{}mb", total * 4096 / 1024 / 1024);
    };

    // Test BitMaps
    kprintln!("Creating test BitMap");
    let mut test_buf = [0u8; 20];
    let mut bmap = Bitmap::<u8>::new(test_buf.as_mut_ptr(), 20, test_buf.len());
    bmap.set(12, true);
    kprintln!("{}", bmap[12]);
}
