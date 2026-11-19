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
        
        #[arg(short = 'c', long)]
        case_insensitive: bool,

        #[arg(short = 'i', long)]
        line_numbers: bool,
    },
    Counter{
        path: std::path::PathBuf,
    }
}

fn grep(pattern: &str, path: &std::path::Path, line_numbers: bool, case_insensitive: bool) -> Result<()>{
    let content: String = std::fs::read_to_string(path).with_context(|| format!("Couldn't read file `{}`", path.display()))?;
    let mut count: i32 = 0;
    let mut printed_line: String = String::new();

    for line in content.lines() {
        let matches = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {

        line.contains(pattern)
    };

    if matches {
        printed_line = format!("{}", line);

        if line_numbers {
            printed_line = format!("{}: {}", count, printed_line);
        }

        println!("{}", printed_line);
    }

    count += 1;
}
    Ok(())
}

fn Counter(path: &std::path::Path) -> Result<()> {
    let content: String = std::fs::read_to_string(path).expect("Couldn't read file");
    let mut last_char: char = ' ';
    let mut word_count: i32 = 0;

    for character in content.chars() {
        if !character.is_whitespace() && last_char.is_whitespace(){
            word_count += 1;
        }
        last_char = character;
    }

    println!("There are {} words", word_count);

    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Grep { pattern, path, line_numbers, case_insensitive} => grep(&pattern, &path, line_numbers, case_insensitive)?,
        Commands::Counter { path } => Counter(&path)?,
    }

    Ok(())
}   
