use regex::Regex;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const NO_OPACITY: &str = "1.0";
const SMALL_OPACITY: &str = "0.9";
const ALACRITY_PATH: &str = ".config/alacritty/alacritty.yml";

fn main() -> std::io::Result<()> {
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();

    let alacritty_config = Path::new(&home).join(ALACRITY_PATH).display().to_string();

    let file = match load_alacritty_config(&alacritty_config) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let content = read_file_content(file)?;

    let (regex, current_value) = capture_regex(&content);

    let new_opacity = assign_new_opacity(current_value);

    let mut f = File::create(&alacritty_config)?;
    let after = regex.replace_all(&content, format!("opacity: {}", new_opacity));
    f.write_all(after.as_bytes())?;

    f.sync_all()?;

    println!("Swapped to new opacity value: {}", new_opacity);
    Ok(())
}

fn assign_new_opacity(current_value: &str) -> &str {
    if current_value.trim() == NO_OPACITY {
        SMALL_OPACITY
    } else {
        NO_OPACITY
    }
}

fn capture_regex(content: &str) -> (Regex, &str) {
    let regex = Regex::new("opacity: (.*)").unwrap();
    let current_value = regex.captures(content).unwrap().get(1).unwrap().as_str();
    (regex, current_value)
}

fn read_file_content(file: File) -> Result<String, std::io::Error> {
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn load_alacritty_config(alacritty_config: &String) -> Result<File, Result<(), std::io::Error>> {
    let file = match File::open(alacritty_config) {
        Ok(it) => it,
        Err(err) => return Err(Err(err)),
    };
    Ok(file)
}
