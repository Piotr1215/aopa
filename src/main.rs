use std::{io, process::exit};

fn main() -> io::Result<()> {
    let home = env!("HOME");
    dbg!(home);

    if home.is_empty() {
        eprintln!("Variable HOME is not defined, please define");
        exit(1);
    }

    let file = std::fs::File::options()
        .open("/home/decoder/dev/rust/aopa/costam.yml")
        .unwrap();

    let mut value: serde_yaml::Value = serde_yaml::from_reader(&file).unwrap();

    let opacity = &value.get("window").unwrap().get("opacity");
    dbg!(opacity);

    if serde_yaml::to_string(opacity).unwrap() == "0.9" {
        *value.get_mut("window").unwrap().get_mut("opacity").unwrap() = "1.0".into();
    } else {
        *value.get_mut("window").unwrap().get_mut("opacity").unwrap() = "0.9".into();
    }

    serde_yaml::to_writer(file, &value).unwrap();

    Ok(())
}
