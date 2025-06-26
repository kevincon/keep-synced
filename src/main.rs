use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Path to one or more files or directories to synchronize
    path: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("You gave me:");
    for file in args.path {
        println!("{}", file.display());
    }

    Ok(())
}
