use std::io::{self, Write};

use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut buffer = String::new();
    let limit_reset_buffer = 100;

    loop {
        if let Event::Key(event) = read()? {
            if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                break;
            }
            if let KeyCode::Char(c) = event.code {
                buffer.push(c);
                print!("{}", c);
                io::stdout().flush()?;

                if buffer.len() >= limit_reset_buffer {
                    println!("cleaning buffer: {}", buffer);
                    buffer.clear();
                    println!("{limit_reset_buffer}");
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
