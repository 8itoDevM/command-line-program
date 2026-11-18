use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Grep{
        pattern: String,
        path: std::path::PathBuf,
    },
}

fn grep(pattern: &str, path: &std::path::Path) -> Result<()>{
    let content: String = std::fs::read_to_string(path).with_context(|| format!("Couldn't read file `{}`", path.display()))?;

    for line in content.lines(){
        if line.contains(pattern){
            println!("{}", line)
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Grep { pattern, path } => grep(&pattern, &path)?,
    }

    Ok(())
}   
