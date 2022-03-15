use clap::Parser;
use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Parser)]
struct Args {
    /// File name of the roster
    file: String,

    /// First name
    name: String,
}

fn main() -> Result<(), io::Error> {
    println!("Check Roster");

    let args = Args::parse();

    let mut file = File::open(args.file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let names = buf.lines().collect::<Vec<&str>>();

    if names.contains(&args.name.as_str()) {
        println!("{} has walked on the moon.", args.name);
    } else {
        println!("{} has NOT walked on the moon!", args.name);
    }

    Ok(())
}
