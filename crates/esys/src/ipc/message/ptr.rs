use crate::{assert_msg_size_is_correct, implement_message_struct};

implement_message_struct! {
    pub struct MessagePointer1 {
        pub addr1: u64,
        pub padding: [u64; 6],
    }
}

implement_message_struct! {
    pub struct MessagePointer2 {
        pub addr1: u64,
        pub addr2: u64,
        pub padding: [u64; 5],
    }
}

implement_message_struct! {
    pub struct MessagePointer3 {
        pub addr1: u64,
        pub addr2: u64,
        pub addr3: u64,
        pub padding: [u64; 4],
    }
}

implement_message_struct! {
    pub struct MessagePointer4 {
        pub addr1: u64,
        pub addr2: u64,
        pub addr3: u64,
        pub addr4: u64,
        pub padding: [u64; 3],
    }
}
