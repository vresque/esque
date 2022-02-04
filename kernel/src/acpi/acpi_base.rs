use core::ptr::NonNull;

use super::SDTHeader;

pub trait ACPIFindable<'name>: Sized + ACPITable {
    const NAME: &'name str;

    fn find<'lt>(sdt: &SDTHeader) -> Option<&'lt Self> {
        sdt.find_table()
    }

    fn find_mut<'lt>(sdt: &SDTHeader) -> Option<&'lt mut Self> {
        sdt.find_table_mut()
    }
}

pub trait ACPITable: Sized {
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
}

#[macro_export]
macro_rules! impl_acpi_table {
    (
        $(
            $stru_name:ident
        )*
    ) => {
        $(
            impl ACPITable for $stru_name {}
        )*
    };
}

#[macro_export]
macro_rules! impl_acpi_findable {
    (
        $(
            $stru_name:ident -> $acpi_name:expr
        )*
    ) => {
        $(
            impl ACPITable for $stru_name {}
            impl<'name> ACPIFindable<'name> for $stru_name {
                const NAME: &'name str = $acpi_name;
            }
        )*
    };
}
