use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "SpikeQ")]
#[command(version = "1.0")]
#[command(author = "Nicholas D. Crosbie")]
#[command(about = "Generates sequences with optional spiked patterns")]
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
        help = "Sets the sequence length range in the form X,Y",
        value_parser = parse_length_range,
        default_value = "100,600"
    )]
    pub length: (usize, usize),

    #[arg(
        short,
        long,
        help = "Sets the forbidden patterns file to use",
        required = false
    )]
    pub forbidden_patterns: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generates sequences with spiked patterns")]
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
