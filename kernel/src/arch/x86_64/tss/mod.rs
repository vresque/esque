use crate::arch::gdt::Ring;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
/// Help: https://wiki.osdev.org/Task_State_Segment
pub struct Tss {
    pub _reserved: u32,
    pub rsp: [u64; 3],
    pub _reserved_2: u64,
    pub ist: [u64; 7],
    pub _reserved_3: u64,
    pub _reserved_4: u64,
    pub io_base: u16,
}

impl Default for Tss {
    fn default() -> Self {
        Self {
            _reserved: 0,
            rsp: [0; 3],
            _reserved_2: 0,
            ist: [0; 7],
            _reserved_3: 0,
            _reserved_4: 0,
            io_base: 0,
        }
    }
}

impl Tss {
    pub const fn new(rsp: [u64; 3], ist: [u64; 7], io_base: u16) -> Self {
        Self {
            _reserved: 0,
            rsp,
            _reserved_2: 0,
            ist,
            _reserved_3: 0,
            _reserved_4: 0,
            io_base,
        }
    }

    pub fn set_ist(&mut self, idx: usize, stack: u64) {
        if idx > 6 {
            panic!("index out of bounds");
        }
        self.ist[idx] = stack;
    }

    pub fn set_rsp(&mut self, ring: Ring, stack: u64) {
        if ring != Ring::Ring0 {
            panic!("cannot set rsp for ring3");
        }
        self.rsp[0] = stack;
    }
}

#[repr(align(16), C)]
pub struct Tss64(Tss);

pub struct ProcessorControl {
    pub tcb: usize,
    pub user_rsp: usize,
    pub tss: Tss64,
}

impl ProcessorControl {
    pub const fn new(tcb: usize, user_rsp: usize, tss: Tss) -> Self {
        Self {
            tcb,
            user_rsp,
            tss: Tss64(tss),
        }
    }
}

#[thread_local]
pub static mut PROC_CONT: ProcessorControl =
    ProcessorControl::new(0, 0, Tss::new([0; 3], [0; 7], 0xFFFF));

pub unsafe fn set_tss_stack(ptr: u64) {
    PROC_CONT.tss.0.rsp[0] = ptr;
}
