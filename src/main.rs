use regex::Regex;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

const NO_OPACITY: &str = "1.0";
const SMALL_OPACITY: &str = "0.9";
const ALACRITY_PATH: &str = ".config/alacritty/alacritty.yml";

fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        println!("This will not work on windows, get Linux instead!");
        process::exit(1)
    }

    #[allow(deprecated)]
    let home = env::home_dir().unwrap();

    let alacritty_config = Path::new(&home).join(ALACRITY_PATH).display().to_string();

    let file = File::open(&alacritty_config)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let regex = Regex::new("opacity: (.*)").unwrap();
    let current_value = regex.captures(&content).unwrap().get(1).unwrap().as_str();
    let mut f = File::create(&alacritty_config)?;

    let new_opacity = if current_value.trim() == NO_OPACITY {
        SMALL_OPACITY
    } else {
        NO_OPACITY
    };

    let after = regex.replace_all(&content, format!("opacity: {}", new_opacity));
    f.write_all(after.as_bytes())?;

    f.sync_all()?;

    println!("Swapped to new opacity value: {}", new_opacity);
    Ok(())
}
