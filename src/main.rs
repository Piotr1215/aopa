use std::{
    env,
    path::Path,
    process::{self, Command},
};

const NO_OPACITY: &str = "1.0";
const SMALL_OPACITY: &str = "0.9";
const ALACRITY_PATH: &str = ".config/alacritty/alacritty.yml";
const YQ_PATH: &str = "/usr/bin/yq";
const SED_PATH: &str = "/usr/bin/sed";

fn main() {
    if cfg!(target_os = "windows") {
        println!("This will not work on windows, get Linux instead!");
        process::exit(1)
    }

    if !Path::new(YQ_PATH).exists() || !Path::new(SED_PATH).exists() {
        print!("yq or sed are not present in the system, exiting");
        process::exit(1)
    }

    #[allow(deprecated)]
    let home = env::home_dir().unwrap();

    let alacritty_config = Path::new(&home).join(ALACRITY_PATH).display().to_string();

    let output = Command::new(YQ_PATH)
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

    Command::new(SED_PATH)
        .arg("-i")
        .arg(sub)
        .arg(alacritty_config)
        .spawn()
        .expect("failed to execute process");

    println!("Swapped to new opacity value: {}", new_opacity);
}
