use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Paths to one or more files or directories containing files to synchronize
    #[clap(default_values_os_t = vec![std::env::current_dir().unwrap()])]
    paths: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("You gave me:");
    for file in args.paths {
        println!("{}", file.display());
    }

    Ok(())
}
