<!-- <img src="src/grepq-icon.svg" width="128" /> -->

_Generates synthetic FASTQ records free of forbidden regex patterns, or containing sequences with 'spiked' regex patterns_

<!-- [![Crates.io](https://img.shields.io/crates/v/grepq.svg)](https://crates.io/crates/grepq)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) -->

## Feature set

- Generates FASTQ records with random DNA sequences of specified lengths.
- Ensure sequences do not contain forbidden regex patterns.
- Insert specific regex patterns into a subset of sequences.
- Generate a summary of inserted patterns and their counts.

## Usage 
Get instructions and examples using `spikeq -h`, and `spikeq spike-sequence -h` for help on the `spike-sequence` subcommand.

The regex patterns should only include the DNA sequence characters (A, C, G, T), and not IUPAC ambiguity codes (N, R, Y, etc.). If your regex patterns contain any IUPAC ambiguity codes, then transform them to DNA sequence characters (A, C, G, T) before using them with `spikeq`. See `regex.json` in the `examples` directory for an example of valid pattern file.

## Requirements

- Rust
- Cargo

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/dna-sequence-generator.git
    cd dna-sequence-generator
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the project:
    ```sh
    cargo run --release -- <arguments>
    ```

### Command-Line Arguments

- `--num_sequences <NUM>`: Number of sequences to generate.
- `--length <MIN> <MAX>`: Minimum and maximum length of the sequences.
- `--forbidden_patterns <FILE>`: JSON file containing forbidden patterns.
- `--num_sp_sequences <NUM>`: Number of sequences to spike with patterns.
- `--patterns <FILE>`: JSON file containing patterns to insert.

### Example

```sh
cargo run --release -- --num_sequences 100 --length 100 150 --forbidden_patterns forbidden.json --num_sp_sequences 10 --patterns patterns.json
```

## Output

The generated sequences and quality lines are saved in a text file with a unique identifier as the filename. A summary of the inserted patterns and their counts is saved in `inserted.json`.

## License

This project is licensed under the MIT License.
```