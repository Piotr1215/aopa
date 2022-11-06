use std::{
    env,
    path::Path,
    process::{self, Command},
};

const NO_OPACITY: &str = "1.0";
const SMALL_OPACITY: &str = "0.9";

fn main() {
    if cfg!(target_os = "windows") {
        println!("This will not work on windows, get Linux instead!");
        process::exit(1)
    }

    if !Path::new("/usr/bin/yq").exists() || !Path::new("/usr/bin/sed").exists() {
        print!("yq or sed are not present in the system, exiting");
        process::exit(1)
    }

    #[allow(deprecated)]
    let home = env::home_dir().unwrap();

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

    let new_opacity = if current_opacity.trim() == NO_OPACITY {
        SMALL_OPACITY
    } else {
        NO_OPACITY
    };

    let sub: String = format!(r#"s#  opacity: \(.*\)#  opacity: {}#"#, new_opacity);

    Command::new("/usr/bin/sed")
        .arg("-i")
        .arg(sub)
        .arg(alacritty_config)
        .spawn()
        .expect("failed to execute process");

    println!("Swapped to new opacity value: {}", new_opacity);
}
