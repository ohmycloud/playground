use std::path::PathBuf;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let input_path = args.input_path;
    let sheet_name = args.sheet_name;

    println!("input-path={:?}, sheet_name={:?}", input_path, sheet_name);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "read Excel with Polars")]
struct Cli {
    /// Path of input file
    #[arg(short, long)]
    input_path: PathBuf,
    /// Worksheet name
    #[arg(short, long)]
    sheet_name: String,
}
