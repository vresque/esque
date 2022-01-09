pub mod message;
use message::*;

use crate::process::Process;

#[repr(C)]
#[derive(Copy, Clone)]
pub union MessageContent {
    pub message_8: numeric::MessageU8,
    pub message_16: numeric::MessageU16,
    pub message_32: numeric::MessageU32,
    pub message_64: numeric::MessageU64,
    pub ptr1: ptr::MessagePointer1,
    pub ptr2: ptr::MessagePointer2,
    pub ptr3: ptr::MessagePointer3,
    pub ptr4: ptr::MessagePointer4,
    // At most 56 bytes
    _size: [u8; 56],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Message {
    pub source: Process,
    pub destination: Process,
    pub typ: i32,
    pub content: MessageContent,
}

impl Message {
    pub fn new<T>(source: Process, destination: Process, typ: T, content: MessageContent) -> Self
    where
        T: Into<i32>,
    {
        Self {
            source,
            destination,
            typ: (typ.into()),
            content,
        }
    }
}
