use keylogger_rs::{disable_raw_mode, enable_raw_mode, run_aplication};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    run_aplication()?;
    disable_raw_mode()?;
    Ok(())
}
