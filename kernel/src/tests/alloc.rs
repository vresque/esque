use esqtest::*;
use esys::ipc::IPCQueueHeader;

use crate::{address_of, heap::malloc};

#[esqtest::test]
pub fn check_allocation() {
    check_neq!(address_of!(malloc::<IPCQueueHeader>()), 0);
    for i in 0..10 {
        check_neq!(address_of!(malloc::<u64>()), 0);
    }
    all_good!()
}
