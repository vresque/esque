#![no_std]
#![feature(concat_idents)]

#[macro_export]
macro_rules! module_info {
    (
        name: $nam:ident,
        description: $desc:expr,
        maintainer: $maintainer:expr,
        license: $license:expr,
        safe: $safe:expr,
     ) => {
        use crate::module_tag;
        module_tag!(name, $nam);
        module_tag!(description, $desc);
        module_tag!(maintainer, $maintainer);
        module_tag!(license, $license);
        module_tag!(issafe, $safe);
    };
}

#[macro_export]
macro_rules! __module_tag {
    ($tag:ident, $name:expr, $val:expr) => {
        //#[link_section = ".modinfo"]
        //#[align(1)]
        //pub static concat_idents!(___MODULE_TAG_, $tag): [u8] = concat!($name, stringify!($val)).as_bytes()
    };
}

#[macro_export]
macro_rules! module_tag {
    ($tag:ident, $val:expr) => {
        crate::__module_tag!($tag, stringify!($tag), $val);
    };
}

pub fn test() {
    module_tag!(lmao_fuck_you, 23);
}
