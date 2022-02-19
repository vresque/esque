use alloc::boxed::Box;
use esqtest::all_good;
use esqtest::*;

use crate::{
    debug,
    env::{getenv, setenv},
};

#[esqtest::test]
pub fn test_environment_variables() {
    setenv("SHELL", "/bin/sh");
    setenv("KERNEL_VERSION", "0.1-rc1");
    setenv("LICENSE", "GPLv2");
    setenv("KERNEL", "esque");
    setenv("HELLO", "HELLO WORLD!!");

    let shell = getenv("SHELL");
    debug!("{:?}", shell);
    check!(shell.is_some());
    check_eq!(&*shell.unwrap(), "/bin/sh");

    let kernel = getenv("KERNEL");
    debug!("{:?}", kernel);
    check!(kernel.is_some());
    check_eq!(&*kernel.unwrap(), "esque");

    let version = getenv("KERNEL_VERSION");
    debug!("{:?}", version);
    check!(version.is_some());
    check_eq!(&*version.unwrap(), "0.1-rc1");

    let license = getenv("LICENSE");
    debug!("{:?}", license);
    check!(license.is_some());
    check_eq!(&*license.unwrap(), "GPLv2");

    let hello = getenv("HELLO");
    debug!("{:?}", hello);
    check!(hello.is_some());
    check_eq!(&*hello.unwrap(), "HELLO WORLD!!");

    setenv("SHELL", "/bin/bash");
    let shell_again = getenv("SHELL");
    debug!("{:?}", shell_again);
    check!(shell_again.is_some());
    check_eq!(&*shell_again.unwrap(), "/bin/bash");

    all_good!()
}
