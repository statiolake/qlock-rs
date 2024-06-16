use std::{fs::write, process::Command};

pub fn main() {
    let child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("quine")
        .output()
        .unwrap();

    write("quine_output.rs", child.stdout).unwrap();

    Command::new("diff")
        .arg("-u")
        .arg("src/quine.rs")
        .arg("quine_output.rs")
        .status()
        .unwrap();

    Command::new("sha256sum")
        .arg("src/quine.rs")
        .arg("quine_output.rs")
        .status()
        .unwrap();
}
