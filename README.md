<div align="center">
  <h1>🎯 Everybody Codes Rust Template 🎯</h1>

  <p><b>Starter template for solving <a href="https://everybody.codes/">Everybody Codes</a> in <a href="https://www.rust-lang.org/">Rust</a>.</b></p>

  ![License](https://img.shields.io/github/license/finnhartshorn/everybody-codes-rust)
  ![Rust](https://img.shields.io/badge/rust-latest-orange)

</div>

---

## Features

- **Fast scaffolding**: Quickly create solution files for each quest
- **Automatic input fetching**: Download puzzle inputs and descriptions via [ec-cli](https://github.com/finnhartshorn/ec-cli)
- **Three-part support**: Full support for all three parts of each Everybody Codes quest
- **Built-in testing**: Unit tests with sample inputs for each part
- **Benchmarking**: Track and compare solution performance
- **Auto-submit**: Submit solutions directly from the command line

## Setup

### Prerequisites

1. **Rust** - Install from [rust-lang.org](https://www.rust-lang.org/tools/install)
2. **ec-cli** - Install from [finnhartshorn/ec-cli](https://github.com/finnhartshorn/ec-cli)

### Installation

1. Clone this repository or use it as a template
2. Configure your Everybody Codes session cookie for ec-cli:
   - Option 1: Set the `EC_COOKIE` environment variable
   - Option 2: Store it in `~/.everybodycodes.cookie`
   - Option 3: Use your platform's config directory (e.g., `~/.config/everybodycodes/cookie`)

   To get your session cookie, log into [everybody.codes](https://everybody.codes/), open your browser's developer tools, and find the session cookie value.

3. Set the year in `.cargo/config.toml`:
   ```toml
   [env]
   EC_YEAR = "2025"  # Update to your target year
   ```

## Usage

### Scaffolding a new quest

```sh
# Create solution file and empty input/sample files
cargo scaffold <day>

# Scaffold and immediately download inputs
cargo scaffold <day> --download
```

This creates:
- `src/bin/<day>.rs` - Solution file with 3 parts
- `data/inputs/<day>-1.txt`, `<day>-2.txt`, `<day>-3.txt` - Input files
- `data/samples/<day>-1.txt`, `<day>-2.txt`, `<day>-3.txt` - Sample files for testing

### Downloading inputs

```sh
# Download inputs, samples, and description for a quest
cargo download <day>
```

This fetches all three parts automatically using ec-cli.

### Running solutions

```sh
# Run all parts for a quest
cargo solve <day>

# Run in release mode (faster)
cargo solve <day> --release

# Run and submit a specific part
cargo solve <day> --release --submit <part>
```

### Testing

```sh
# Test a specific quest
cargo test --bin <day>

# Test all quests
cargo test
```

### Benchmarking

```sh
# Benchmark a single quest
cargo time <day>

# Benchmark all quests
cargo time --all

# Save results to README
cargo time --all --store
```

### Reading puzzle descriptions

```sh
# Display the puzzle description
cargo read <day>
```

### Today's quest (if available)

```sh
# Scaffold, download, and read today's quest
cargo today
```

## Project Structure

```
.
├── .cargo/
│   └── config.toml          # Cargo aliases and environment
├── data/
│   ├── inputs/              # Puzzle inputs (*.txt per part)
│   ├── samples/             # Sample test cases
│   └── descriptions/        # Puzzle descriptions (HTML)
├── src/
│   ├── bin/                 # Individual quest solutions (01.rs, 02.rs, ...)
│   ├── template/            # Shared utilities and CLI
│   │   ├── commands/        # Command implementations
│   │   ├── ec_cli.rs        # ec-cli wrapper
│   │   └── ...
│   ├── lib.rs
│   ├── main.rs              # CLI entry point
│   └── template.txt         # Quest template
└── Cargo.toml
```

## Solution Template

Each quest follows this structure:

```rust
everybody_codes::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

pub fn part_three(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&everybody_codes::template::read_file("samples", DAY, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&everybody_codes::template::read_file("samples", DAY, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&everybody_codes::template::read_file("samples", DAY, 3));
        assert_eq!(result, None);
    }
}
```

## Benchmarks

<!--- benchmarking table --->
<!--- benchmarking table --->

## Credits

This template is inspired by and adapted from:
- [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) - The excellent AoC Rust template
- [finnhartshorn/ec-cli](https://github.com/finnhartshorn/ec-cli) - CLI tool for Everybody Codes

## License

This project is released into the public domain under the [Unlicense](LICENSE).

The Everybody Codes challenges are created and owned by the Everybody Codes team.
