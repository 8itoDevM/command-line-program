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

        #[arg(short = 'c', long)]
        count_characters: bool,

        #[arg(short = 'b', long)]
        count_bytes: bool,
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

fn Counter(path: &std::path::Path, count_characters: bool, count_bytes: bool) -> Result<()> {
    let content: String = std::fs::read_to_string(path).expect("Couldn't read file");
    let mut last_char: char = ' ';
    let mut word_count: i32 = 0;
    let mut character_count: i32 = 0;

    for character in content.chars() {
        character_count += 1;
        if !character.is_whitespace() && last_char.is_whitespace(){
            word_count += 1;
        }
        last_char = character;
    }

    print!("There are {} words ", word_count);

    if count_characters {
        print!("and {} characters (with white spaces) ", character_count);
    }

    if count_bytes {
        let metadata = std::fs::metadata(path)?;
        let mut size_info: response;

        struct response {
            file_uom: String,
            converted_size: f64,
        }

        match metadata.len() {
            size if size >= 1073741824 => size_info = response { file_uom: String::from("GiB"), converted_size: size as f64/1073741824.0, },
            size if size >= 1048576 => size_info = response { file_uom: String::from("MiB"), converted_size: size as f64/1048576.0, },
            size if size >= 1024 => size_info = response { file_uom: String::from("KiB"), converted_size: size as f64/1024.0, },
            _ => size_info = response { file_uom: String::from("bytes"), converted_size: metadata.len() as f64, },
        }
        print!("and has a file size of {:.2} {} ", size_info.converted_size, size_info.file_uom);
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Grep { pattern, path, line_numbers, case_insensitive} => grep(&pattern, &path, line_numbers, case_insensitive)?,
        Commands::Counter { path, count_characters, count_bytes } => Counter(&path, count_characters, count_bytes)?,
    }

    Ok(())
}   
