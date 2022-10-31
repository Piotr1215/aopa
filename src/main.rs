use std::{path::Path, process::Command};

fn main() {
    println!("{}", Path::new("/usr/bin/seda").exists());
    // TODO: Error handling for executables
    let output = Command::new("/usr/bin/yq")
        .arg("eval")
        .arg(".window.opacity")
        // TODO: make sure file exists
        .arg("costam.yml")
        .output()
        // TODO: Error handling with `Result`
        .expect("failed to execute process");

    println!("Opacity: {}", String::from_utf8_lossy(&output.stdout));

    Command::new("/usr/bin/sed")
        .arg("-i")
        .arg("s#  opacity: \\(.*\\)#  opacity: 1.0#")
        .arg("costam.yml")
        .spawn()
        .expect("failed to execute process");
}
