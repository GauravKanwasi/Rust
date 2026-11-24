use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const INTERVAL: Duration = Duration::from_millis(800);

fn main() -> io::Result<()> {
    let mut out = io::stdout().lock();

    for n in 0u64.. {
        writeln!(out, "tick {n}")?;
        out.flush()?;
        thread::sleep(INTERVAL);
    }

    Ok(())
}
