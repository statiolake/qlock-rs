use std::process::Command;

pub fn main() {
    super::compile::main();
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("qlock")
        .status()
        .unwrap();
}
