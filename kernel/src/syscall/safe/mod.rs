pub enum SafeResult<'a> {
    PointerIsNone,
    BadElemCount,
    BadSize,
    OtherError(&'a str),
}

#[repr(C, packed)]
pub struct Safe {
    kind: u64,
    size: usize,
    ty: u8,
    elem: usize,
    address: u64,
}

enumtastic::enum_with_options! {
    pub enum SafeType: u64 => {
        Some = 0,
        None = 1,
        SomeArray = 2,
    }
}

enumtastic::enum_with_options! {
    pub enum SyscallPtrType: u8 => {
        Void = 0,
        Int = 1,
        Float = 2,
        FileHandle = 3,
    }
}

impl SafeType {
    pub fn custom(n: u64) -> SafeType {
        SafeType(n)
    }
}

impl Safe {
    pub unsafe fn ptr<T>(&self) -> *const T {
        self.address as *const u64 as *const T
    }

    pub unsafe fn ptr_mut<T>(&self) -> *mut T {
        self.address as *mut u64 as *mut T
    }

    pub fn get<T>(&self) -> Result<&T, SafeResult> {
        if self.kind == SafeType::None.0 {
            return Err(SafeResult::PointerIsNone);
        }

        unsafe { Ok(&*(self.address as *const u64 as *const T)) }
    }

    //pub fn get_mut<T>(&mut self) -> Result<&mut T, SafeResult> {
    //    let val = &mut *self.get()?;
    //    Ok(val)
    //}

    pub fn from_addr(
        addr: u64,
        array: bool,
        size: usize,
        ptr_ty: SyscallPtrType,
        elem: usize,
    ) -> Safe {
        let mut ty = if array {
            SafeType::SomeArray
        } else {
            SafeType::Some
        };

        if (addr as *const u64) == core::ptr::null::<u64>() {
            ty = SafeType::None;
        }

        Safe {
            kind: ty.0,
            size: size,
            ty: ptr_ty.0,
            elem: elem,
            address: addr,
        }
    }
}
