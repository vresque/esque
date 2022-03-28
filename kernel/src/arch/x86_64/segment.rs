use crate::arch::gdt::Ring;
use core::arch::asm;

bitflags::bitflags! {
    pub struct Segment: u16 {
        const RING0 = 0b00;
        const RING1 = 0b01; // Do not use: Only ring0 and ring3 are supported
        const RING2 = 0b10; // Same as above;
        const RING3 = 0b11;
        const GDT_IS_USED = 0 << 2;
        const LDT_IS_USED = 1 << 2;
    }
}

impl Segment {
    pub const fn raw(bits: u16) -> Self {
        Self { bits }
    }

    pub const fn new(ring: Ring, index: u16) -> Self {
        Self {
            bits: index << 3 | (ring as u16),
        }
    }

    pub const fn index(&self) -> u16 {
        self.bits >> 3
    }
}

pub unsafe fn upload_to_cs(seg: Segment) {
    asm!(
        "push {sel}",
        "lea {temp_r}, [1f + rip]",
        "push {temp_r}",
        "retfq",
        "1:", sel = in(reg) (u64::from(seg.bits())),
        temp_r = lateout(reg) _, // stub
        options(preserves_flags),
    );
}

pub unsafe fn upload_to_ss(seg: Segment) {
    asm!("mov {}, ss", in(reg) seg.bits());
}

pub unsafe fn upload_to_dx(seg: Segment) {
    asm!("mov ax, dx", in("ax") seg.bits());
}

pub unsafe fn upload_to_ds(seg: Segment) {
    asm!("mov {}, ds", in(reg) seg.bits());
}

pub unsafe fn upload_to_es(seg: Segment) {
    asm!("mov {}, es", in(reg) seg.bits());
}

pub unsafe fn upload_to_fs(seg: Segment) {
    asm!("mov {}, fs", in(reg) seg.bits());
}

pub unsafe fn upload_to_gs(seg: Segment) {
    asm!("mov {}, gs", in(reg) seg.bits());
}
