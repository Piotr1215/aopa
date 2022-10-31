use std::process::Command;

fn main() {
    let output = Command::new("/usr/bin/yq")
        .arg("eval")
        .arg(".window.opacity")
        .arg("costam.yml")
        .output()
        .expect("failed to execute process");

    println!("Opacity: {}", String::from_utf8_lossy(&output.stdout));

    Command::new("/usr/bin/sed")
        .arg("-i")
        .arg("s#  opacity: \\(.*\\)#  opacity: 1.0#")
        .arg("costam.yml")
        .spawn()
        .expect("failed to execute process");
}
