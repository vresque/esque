pub fn main() {
    println!("cargo:rerun-if-changed=../.targets/x86_64/kernel.lds")
}
