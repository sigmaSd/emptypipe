use std::env;
use std::io::{self, Read};
use std::process::Command;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        Command::new(env::args().nth(1).expect("No command given"))
            .args(&env::args().skip(2).collect::<Vec<String>>())
            .spawn()?;
    } else {
        print!("{}", &buffer);
    }
    Ok(())
}
