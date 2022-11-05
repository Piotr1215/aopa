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
    let _home = env::var("HOME").unwrap_or_else(|_| "/home/decoder/".to_owned());

    // let alacritty_config = Path::new(&home).join(".config/alacritty/alacritty.yml");
    let alacritty_config = Path::new("/home/decoder/dev/rust/aopa/costam.yml");

    let output = Command::new("/usr/bin/yq")
        .arg("eval")
        .arg(".window.opacity")
        .arg(&alacritty_config)
        .output()
        .expect("failed to execute process");

    let current_opacity = String::from_utf8_lossy(&output.stdout);

    println!("Current opacity {}", current_opacity);

    if current_opacity == "0.9" {
        println!("Current opacity {}", current_opacity);
        Command::new("/usr/bin/sed")
            .arg("-i")
            .arg("s#  opacity: \\(.*\\)#  opacity: 0.9#")
            .arg("costam.yml")
            .spawn()
            .expect("failed to execute process");
    } else {
        println!("Current opacity {}", current_opacity);
        Command::new("/usr/bin/sed")
            .arg("-i")
            .arg("s#  opacity: \\(.*\\)#  opacity: 1.0#")
            .arg("costam.yml")
            .spawn()
            .expect("failed to execute process");
    }
}
