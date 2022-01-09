use crate::{assert_msg_size_is_correct, implement_message_struct};

implement_message_struct! {
    pub struct MessageU8 {
        data: [u8; 56],
    }
}

implement_message_struct! {
    pub struct MessageU16 {
        pub data: [u16; 28],
    }
}

implement_message_struct! {
    pub struct MessageU32 {
        pub data: [u32; 14],
    }
}

implement_message_struct! {
    pub struct MessageU64 {
        pub data: [u64; 7],
    }
}
