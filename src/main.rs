use std::env;
use std::io::{self, Read};
use std::process::Command;

fn main() -> io::Result<()> {
    let inv = match env::args().nth(1) {
        Some(ref flag) if flag == "-i" => true,
        _ => false,
    };

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() && !inv {
        Command::new(env::args().nth(1).expect("No command given"))
            .args(&env::args().skip(2).collect::<Vec<String>>())
            .spawn()?;
    } else if !buffer.trim().is_empty() && inv {
        Command::new(env::args().nth(2).expect("No command given"))
            .args(&env::args().skip(3).collect::<Vec<String>>())
            .spawn()?;
    } else {
        print!("{}", &buffer);
    }
    Ok(())
}
