use clap::Parser;
use std::path::PathBuf;

mod analyzer;

#[derive(Parser, Debug)]
#[command(name = "simple-wc-tool")]
#[command(version = "0.1.0")]
#[command(author = "Vladislav Dyachenko")]
#[command(about = "File content analyzer")]
struct Args {
    file_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match analyzer::analyze_file(&args.file_path) {
        Ok(stats) => {
            println!("Words: {}", stats.words);
            println!("Lines: {}", stats.lines);
            println!("Characters: {}", stats.chars);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
