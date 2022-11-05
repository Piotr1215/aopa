use std::{
    env,
    path::Path,
    process::{self, Command},
};

fn main() {
    if !Path::new("/usr/local/bin/yq").exists() || !Path::new("/usr/bin/sed").exists() {
        print!("yq or sed are not present in the system, exiting");
        process::exit(1)
    }
    let home = env::var("HOME").unwrap();

    let alacritty_config = Path::new(&home)
        .join(".config/alacritty/alacritty.yml")
        .display()
        .to_string();

    let output = Command::new("/usr/bin/yq")
        .arg("eval")
        .arg(".window.opacity")
        .arg(&alacritty_config)
        .output()
        .expect("failed to execute process");

    let current_opacity = String::from_utf8_lossy(&output.stdout);

    let mut new_opacity = String::from("1.0");

    if current_opacity.trim() == new_opacity.trim() {
        new_opacity = String::from("0.9");
    }

    let sub: String = format!("s#  opacity: \\(.*\\)#  opacity: {}#", new_opacity.trim());

    Command::new("/usr/bin/sed")
        .arg("-i")
        .arg(sub)
        .arg(alacritty_config)
        .spawn()
        .expect("failed to execute process");
}
