use std::io::{self, Seek};

use serde_yaml::Value;

fn main() -> io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("sample_file.yaml")
        .expect("File should exist");

    let mut value: Value = serde_yaml::from_reader(&file).unwrap();

    let pa = serde_yaml::to_string(&value["window"]["opacity"]).unwrap();

    let aka = pa.trim().to_string();

    println!("{:?}", &aka);

    let toggle_match = matcher(aka);

    println!("This is after toggle match call {:?}", toggle_match);

    value["window"]["opacity"] = toggle_match.into();

    file.rewind().unwrap();

    serde_yaml::to_writer(&file, &value).unwrap();

    Ok(())
}
fn matcher(x: String) -> String {
    println!("{:?}", x.trim().to_string());
    match x.trim() {
        "'1.0'" => String::from("'0.9'"),
        "'0.9'" => String::from("'1.0'"),
        _ => String::from("'1.0'"),
    }
}
