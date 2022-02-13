use spin::Once;

pub const USERSPACE_STACK_SIZE: u64 = 0x64000;
pub const USERSPACE_ADDRESS_MASK_SHIFT: u64 = 47; // If we ever do lvl 5 paging: 56

pub const USER_STACK_TOP: u64 = 0x7fffffffe000;
pub const USER_STACK_BOTTOM: u64 = USER_STACK_TOP - USERSPACE_STACK_SIZE;

pub fn userspace_get_last_address() -> u64 {
    static ADDRESS: Once<u64> = Once::new();

    *ADDRESS.call_once(|| 1 << USERSPACE_ADDRESS_MASK_SHIFT)
}
