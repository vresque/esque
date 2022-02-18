pub mod env;

pub fn syscall(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    bp: usize,
    stack: &mut usize,
) -> usize {
    10
}
