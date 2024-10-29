use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    file_name: String,
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let file = File::open(&cli.file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = start(cli) {
        println!("Error: {e}");
    }
}
