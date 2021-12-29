#![no_std]

#[macro_export]
macro_rules! module_init {
    (
        name: $nam:expr,
        description: $desc:expr,
        maintainer: $maintainer:expr,
        license: $license:expr,
        safe: $safe:expr,
     ) => {
        pub const __MODULE_NAME: &'static str = $nam;
        pub const __MODULE_DESCRIPTION: &'static str = $desc;
        pub const __MODULE_MAINTAINER: &'static str = $maintainer;
        pub const __MODULE_LICENSE: &'static str = $license;
        pub const __MODULE_IS_SAFE: bool = $safe;
    }
}