use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "SpikeQ")]
#[command(version = "1.0")]
#[command(author = "Nicholas D. Crosbie")]
#[command(about = "Generates sequences with optional spiked patterns")]
pub struct Args {
    #[arg(help = "Sets the config file to use")]
    pub config: String,

    #[arg(
        short,
        long,
        help = "Sets the number of sequences to generate",
        default_value_t = 1
    )]
    pub num_sequences: usize,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generates sequences with spiked patterns")]
    SpikeSequence {
        #[arg(short, long, help = "Number of patterns to spike into sequences")]
        num_patterns: usize,

        #[arg(short, long, help = "Number of sequences to spike patterns into")]
        num_sequences: usize,
    },
}
