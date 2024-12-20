<img src="src/spikeq-icon.png" width="128" />

_Generates synthetic FASTQ records free of sequences defined by regex patterns, or containing spiked sequences based on regex patterns_

[![Crates.io](https://img.shields.io/crates/v/spikeq.svg)](https://crates.io/crates/spikeq)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Feature set

- Verifies the regex pattern file meets the required format (validation of the pattern file is performed before processing; see the `schema.json` file in the `examples` directory)
- Generates FASTQ records with random DNA sequences of specified lengths, and free from regex patterns specified in the regex pattern file
- Inserts spike patterns derived from a regex set into a subset of sequences using the `spike-sequence` subcommand, resulting in a FASTQ file with a subset of sequences containing the inserted patterns, and a summary file of the inserted patterns

## Usage 

`spikeq` may be used test bioinformatics tools that process FASTQ files, such as `grepq` (https://github.com/Rbfinch/grepq)

Get instructions and examples using `spikeq -h`, and `spikeq spike-sequence -h` for help on the `spike-sequence` subcommand. 

>[!NOTE]
The regex patterns should only include the DNA sequence characters (A, C, G, T), and not IUPAC ambiguity codes (N, R, Y, etc.). If your regex patterns contain any IUPAC ambiguity codes, then transform them to DNA sequence characters (A, C, G, T) before using them with `spikeq`. See `regex.json` in the `examples` directory for an example of valid pattern file.

## Requirements

- `spikeq` has been tested on Linux and macOS. It might work on Windows, but it has not been tested on this platform.
- Ensure that Rust is installed on your system (https://www.rust-lang.org/tools/install)
- If the build fails, make sure you have the latest version of the Rust compiler by running `rustup update`

## Installation
- From *crates.io* (easiest method)
    - `cargo install spikeq`

- From *source*
    - Clone the repository and `cd` into the `spikeq` directory
    - Run `cargo build --release`
    - Relative to the cloned parent directory, the executable will be located in `./target/release`
    - Make sure the executable is in your `PATH` or use the full path to the executable


## Examples

```sh
# Generate 1000 synthetic FASTQ records with sequence lengths between 200 and 800, and which are free from the regex patterns specified in the regex.json file (generated the FASTQ file named `459cac6f-8d65-48ed-99aa-f03930b3c02f.fastq`).
spikeq -r regex.json -n 1000 -l 200,800

# Generate 1000 synthetic FASTQ records with sequence lengths between 200 and 800, and which are free from the regex patterns specified in the regex.json file, then insert two patterns generated from the regex.json file into 10 sequences (generated the FASTQ file named `4b1f92dc-14e1-496f-a68b-d1683251d827.fastq`, and the summary file named `inserted.json` ).
spikeq -r regex.json -n 1000 -l 200,800 spike-sequence --num-patterns 2 --num-sequences 10
```

## Citation

If you use `spikeq` in your research, please cite as follows:

Crosbie, N.D. (2024). spikeq: A synthetic FASTQ record generator with pattern spiking. 10.5281/zenodo.14211052.

## Update changes

see [CHANGELOG](https://github.com/Rbfinch/spikeq/blob/main/CHANGELOG.md)

## Logo attribution

The logo was created using [Inkscape](https://inkscape.org/) and is based on the Thorn Helix SVG Vector at SVGRepo (https://www.svgrepo.com/svg/321583/thorn-helix).

## License

MIT
