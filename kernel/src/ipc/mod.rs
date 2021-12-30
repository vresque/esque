#[repr(C, packed)]
pub struct Message {
    src: Process,
    dst: Process,
    type_id: u64,
    body: u64,
}