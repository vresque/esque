pub mod common;
pub mod config;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod pic;
pub mod pit;
pub mod userspace;

esqdev::module_init! {
    name: "initializer",
    description: "Initializes the Kernel.",
    maintainer: "<empty>",
    license: "GPL-2",
    safe: false,
}
