#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptFrame {
    pub _ignore: u64,
}
