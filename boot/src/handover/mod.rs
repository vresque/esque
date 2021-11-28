use bks::Framebuffer;
use bks::Handover;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::Handle;
use log::info;

pub fn init_gop(handle: Handle, mut table: &SystemTable<Boot>) -> Framebuffer {
    let gop = unsafe {
        &mut *(table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()
            .expect_success("Failed to locate GOP")
            .get())
    };

    Framebuffer::new(
        gop.frame_buffer().as_mut_ptr(),
        gop.frame_buffer().size(),
        gop.current_mode_info().resolution().0,
        gop.current_mode_info().resolution().1,
        gop.current_mode_info().stride(),
    )
}

pub fn create_handover(handle: Handle, mut table: &SystemTable<Boot>) -> Handover {
    let framebuffer = init_gop(handle, table);
    info!("{}", framebuffer);
    Handover::new(framebuffer)
}
