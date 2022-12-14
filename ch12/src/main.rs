use std::{env, fs, io, process};

use ch12::Config;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1)
    });

    let content = fs::read_to_string(config.filename)?;

    for line in content.lines() {
        if line.contains(&config.query) {
            println!("{}", line)
        }
    }

    Ok(())
}
