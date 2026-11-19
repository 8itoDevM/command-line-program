# Command-line Program (Rust)

A simple command-line application written in Rust that provides text searching and file counting functionalities.

## Features

- **Grep:** Search for specific words or patterns inside text files.
  - Case-sensitive and case-insensitive searches.
  - Optional line numbering for matching results.
- **Counter:** Count words in a text file.
  - Optional character counting, including whitespace.
  - Optional file size reporting in bytes, KiB, MiB, or GiB.

## Installation

### Windows

1. Download the latest `command-line-program.exe` from the repository's [Releases](../../releases) page, or clone the repository and build it yourself.
2. If building from source, make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
3. Open a terminal in the project directory and run:

```powershell
cargo build --release
```

4. The executable will be generated inside:

```text
target/release/command-line-program.exe
```

You can run it through PowerShell or Command Prompt.

**Note:** Running the executable directly by double-clicking it will close the terminal as soon as the program finishes. Use a terminal to see the output.

## Usage

All commands are executed through the terminal.

### Grep

Search for a specific pattern inside a file.

```powershell
.\command-line-program.exe grep <pattern> <path> [options]
```

| Option | Description |
|---|---|
| `-c`, `--case-insensitive` | Ignore uppercase/lowercase differences. |
| `-i`, `--line-numbers` | Display line numbers alongside matching lines. |

**Examples:**

Case-sensitive search:

```powershell
.\command-line-program.exe grep automation "C:\path\to\test_file.txt"
```

Case-insensitive search with line numbers:

```powershell
.\command-line-program.exe grep file "C:\path\to\test_file.txt" -c -i
```

### Counter

Count the words in a text file, with optional character and file size information.

```powershell
.\command-line-program.exe counter <path> [options]
```

| Option | Description |
|---|---|
| `-c`, `--count-characters` | Include the character count (including whitespace). |
| `-b`, `--count-bytes` | Include the file size. |

**Examples:**

Count words:

```powershell
.\command-line-program.exe counter "C:\path\to\test_file.txt"
```

Count words, characters, and file size:

```powershell
.\command-line-program.exe counter "C:\path\to\test_file.txt" -c -b
```

## Test File

The repository includes a `test_file.txt` file that you can use to experiment with both commands.

It contains sample text for testing searches, line numbers, word counting, character counting, and file size reporting.

If you clone the repository, you can use its path directly. For example, from the project's root directory:

```powershell
.\target\release\command-line-program.exe grep file ".\test_file.txt" -c -i
```

## Technologies

- [Rust](https://www.rust-lang.org/)
- [Clap](https://docs.rs/clap/latest/clap/) - Command-line argument parsing.
- [Anyhow](https://docs.rs/anyhow/latest/anyhow/) - Error handling.

## License

This project is free and open-source software licensed under the MIT License.
You are free to use, modify, and distribute this software, including for commercial purposes, provided that the original copyright and license notice are included.
