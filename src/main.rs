mod enums;
mod lexer;

use enums::*;
use clap::Parser;
use std::io::{BufReader, Write};
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "portfolio-entry-creator", version, about = "Portfolio Entry Creator")]
struct Args {
    /// Input File
    #[arg(short = 'i', long, value_name = "INPUT_FILE")]
    input_file: PathBuf,

    /// Output File
    #[arg(short = 'o', long, value_name = "OUTPUT_FILE")]
    output_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{} o, {} i", args.output_file.display(), args.input_file.display());
}
