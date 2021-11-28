use std::process::Command;

fn main() {
    Command::new("make")
        .args(std::env::args())
        .spawn()
        .expect("No program found that fulfills the purpose of `make`");
}
