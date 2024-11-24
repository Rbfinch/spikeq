use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "spikeq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "A synthetic FASTQ record generator with pattern spiking.",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = "
       EXAMPLES:
             - Generate 1000 synthetic FASTQ records with sequence lengths between 200 and 800, and which are free from the regex patterns specified in the regex.json file
                  `spikeq -r regex.json -n 1000 -l 200,800`

             - Generate 1000 synthetic FASTQ records with sequence lengths between 200 and 800, and which are free from the regex patterns specified in the regex.json file, then insert two patterns generated from the regex.json file into 10 sequences
                  `spikeq -r regex.json -n 1000 -l 200,800 spike-sequence --num-patterns 2 --num-sequences 10`

           TIPS:
             - Ensure you have enough storage space for output files.

          NOTES:
             - The regex patterns should only include the DNA sequence characters (A, C, G, T), and not IUPAC ambiguity codes (N, R, Y, etc.). If your regex patterns contain any IUPAC ambiguity codes, then transform them to DNA sequence characters (A, C, G, T) before using them with `spikeq`. See `regex.json` in the `examples` directory for an example of valid pattern file.

             - Regex patterns with look-around and backreferences are not supported.

          CITATION:
          
               If you use `spikeq` in your research, please cite as follows:
             
                  Crosbie, N.D. (2024). spikeq: A synthetic FASTQ record generator with pattern spiking. 10.5281/zenodo.14211052.

Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License."
)]

pub struct Args {
    #[arg(
        short,
        long,
        help = "Sets the number of sequences to generate",
        default_value_t = 1
    )]
    pub num_sequences: usize,

    #[arg(
        short = 'l',
        long = "length",
        help = "Sets the sequence length range in the form <MIN_LENGTH>,<MAX_LENGTH>",
        value_parser = parse_length_range,
        default_value = "100,600"
    )]
    pub length: (usize, usize),

    #[arg(
        short,
        long,
        help = "Sets the regex patterns file to use",
        required = false
    )]
    pub regex_patterns: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generates synthetic FASTQ file containing sequences with spiked patterns")]
    SpikeSequence {
        #[arg(short = 'n', long, help = "Number of patterns to spike into sequences")]
        num_patterns: usize,

        #[arg(short = 's', long, help = "Number of sequences to spike patterns into")]
        num_sequences: usize,
    },
}

fn parse_length_range(s: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid length range: {}", s));
    }
    let min_length = parts[0]
        .parse::<usize>()
        .map_err(|_| format!("Invalid number: {}", parts[0]))?;
    let max_length = parts[1]
        .parse::<usize>()
        .map_err(|_| format!("Invalid number: {}", parts[1]))?;
    if min_length > max_length {
        return Err(format!(
            "Min length cannot be greater than max length: {}",
            s
        ));
    }
    Ok((min_length, max_length))
}
