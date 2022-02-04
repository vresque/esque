use core::ptr::NonNull;

use super::SDTHeader;

pub trait ACPIFindable<'name>: Sized {
    const NAME: &'name str;
    fn new<'lt>(addr: u64) -> Option<&'lt Self> {
        unsafe {
            let ptr = addr as *mut u64 as *mut Self;
            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }
    fn new_mut<'lt>(addr: u64) -> Option<&'lt mut Self> {
        unsafe {
            let ptr = addr as *mut u64 as *mut Self;
            if ptr.is_null() {
                None
            } else {
                Some(&mut *ptr)
            }
        }
    }

    fn find<'lt>(sdt: &SDTHeader) -> Option<&'lt Self> {
        sdt.find_table()
    }

    fn find_mut<'lt>(sdt: &SDTHeader) -> Option<&'lt mut Self> {
        sdt.find_table_mut()
    }
}

#[macro_export]
macro_rules! impl_acpi_findable {
    (
        $(
            $stru_name:ident -> $acpi_name:expr
        )*
    ) => {
        $(
            impl<'name> ACPIFindable<'name> for $stru_name {
                const NAME: &'name str = $acpi_name;
            }
        )*
    }
}
