#![no_std]

pub use esqtest_derive::*;

pub struct RustTest {
    pub func: fn() -> i32,
    pub name: &'static str,
}

#[macro_export]
macro_rules! check {
    ($c:expr) => {
        if !($c) {
            crate::debug!(concat!(
                "\nTesting failed at the following statment: \n",
                file!(),
                ":",
                line!(),
                " ",
                stringify!($c)
            ));
            return 1;
        }
    };
}

#[macro_export]
macro_rules! check_eq {
    ($a:expr, $b:expr) => {
        esqtest::check!($a == $b)
    };
}

#[macro_export]
macro_rules! check_neq {
    ($a:expr, $b:expr) => {
        esqtest::check!($a != $b)
    };
}

#[macro_export]
macro_rules! check_gt {
    ($a:expr, $b:expr) => {
        esqtest::check!($a > $b)
    };
}

#[macro_export]
macro_rules! check_lt {
    ($a:expr, $b:expr) => {
        esqtest::check!($a < $b)
    };
}

#[macro_export]
macro_rules! check_ge {
    ($a:expr, $b:expr) => {
        esqtest::check!($a => $b)
    };
}

#[macro_export]
macro_rules! check_le {
    ($a:expr, $b:expr) => {
        esqtest::check!($a =< $b)
    };
}

#[macro_export]
macro_rules! all_good {
    () => {
        return 0;
    };
}
