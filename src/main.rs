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
        
        #[arg(short = 'i', long)]
        line_numbers: bool
    },
}

fn grep(pattern: &str, path: &std::path::Path, line_numbers: bool) -> Result<()>{
    let content: String = std::fs::read_to_string(path).with_context(|| format!("Couldn't read file `{}`", path.display()))?;
    let mut count: i32 = 0;

    for line in content.lines(){
        if line.contains(pattern){
            if line_numbers == true {
                println!("{}: {}", count, line);
                count += 1;
            } else{
                println!("{}", line)
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Grep { pattern, path, line_numbers} => grep(&pattern, &path, line_numbers)?,
    }

    Ok(())
}   
