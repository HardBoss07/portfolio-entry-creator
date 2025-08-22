mod enums;
mod lexer;

use enums::*;
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

use lexer::Lexer;

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

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Open input file
    let file = File::open(&args.input_file)?;
    let reader = BufReader::new(file);

    // Collect all lines into one big String with newlines
    let text: String = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?   // unwrap Results
        .join("\n");

    let mut lex = Lexer::new(text);
    let tokens = lex.tokenize();

    println!("Tokens: {:?}", tokens);

    Ok(())
}
