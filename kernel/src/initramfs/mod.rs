use bks::Handover;
use tar::tar::*;

use crate::success;

pub fn load_initramfs(handover: &mut Handover) {
    let bytes = handover.initramfs();

    let tar = Tar::from_slice(bytes);
    for entry in tar.iter() {
        success!("{}", entry.filename)
    }
}
