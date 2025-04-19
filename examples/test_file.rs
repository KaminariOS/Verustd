use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Same as OpenOptions::new().write(true).create(true).open(...)
    println!("Opening");
    let mut file = File::options()
        // .write(!true)
        // .create(true)
        .open("example.txt")?;
    println!("Writing");
    file.write_all(b"Options-based write")?;
    Ok(())
}
