use anyhow::{Context, Result};
use clap::Parser;

// Searches for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let content: String = std::fs::read_to_string(&args.path).with_context(|| format!("Couldn't read file `{}`", args.path.display()))?;

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line)
        }
    }

    Ok(())
}   
