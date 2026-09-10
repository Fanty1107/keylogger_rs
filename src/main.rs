use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open("test.txt");

    let mut content = String::new();
    fs::read_to_string(&mut content)?;
    write!(file?, "{} is a piece of std", content)?;
    println!("{content}");
    Ok(())
}
