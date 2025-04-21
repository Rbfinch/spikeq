use clap::CommandFactory;
use clap_complete::shells;
use clap_mangen::Man;
use std::env;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

// Define custom version of Args struct for build.rs
// This avoids dependency on once_cell and colored in build script
#[derive(clap::Parser)]
#[command(
    name = "spikeq",
    author = "Nicholas D. Crosbie",
    version = env!("CARGO_PKG_VERSION"),
    about = "A synthetic FASTQ record generator with pattern spiking.",
    term_width = 80,
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License."
)]
struct Args {
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
        default_value = "100,600"
    )]
    pub length: String,

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

#[derive(clap::Subcommand)]
enum Commands {
    #[command(about = "Generates synthetic FASTQ file containing sequences with spiked patterns")]
    SpikeSequence {
        #[arg(short = 'n', long, help = "Number of patterns to spike into sequences")]
        num_patterns: usize,

        #[arg(short = 's', long, help = "Number of sequences to spike patterns into")]
        num_sequences: usize,
    },
}

fn generate_man_page(outdir: &Path) -> Result<()> {
    let mut app = Args::command();
    app.set_bin_name("spikeq");

    let man = Man::new(app.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let man_dir = outdir.join("man/man1");
    fs::create_dir_all(&man_dir)?;
    let man_path = man_dir.join("spikeq.1");

    fs::write(man_path, buffer)?;

    println!("cargo:warning=Man page generated in {}", man_dir.display());
    Ok(())
}

fn generate_shell_completions(outdir: &Path) -> Result<()> {
    let mut app = Args::command();
    app.set_bin_name("spikeq");

    // Create base completions directory
    let completions_dir = outdir.join("completions");
    println!(
        "cargo:warning=Creating completions directory: {}",
        completions_dir.display()
    );
    fs::create_dir_all(&completions_dir)?;

    // Create shell subdirectories
    let bash_dir = completions_dir.join("bash");
    let zsh_dir = completions_dir.join("zsh");
    let fish_dir = completions_dir.join("fish");

    fs::create_dir_all(&bash_dir)?;
    fs::create_dir_all(&zsh_dir)?;
    fs::create_dir_all(&fish_dir)?;

    // Generate completions to string buffers and write them to files
    // Bash completions
    println!("cargo:warning=Generating Bash completions");
    let mut bash_buf = Vec::new();
    clap_complete::generate(shells::Bash, &mut app.clone(), "spikeq", &mut bash_buf);
    fs::write(bash_dir.join("spikeq.bash"), bash_buf)?;
    println!("cargo:warning=Bash completions generated successfully");

    // Zsh completions
    println!("cargo:warning=Generating Zsh completions");
    let mut zsh_buf = Vec::new();
    clap_complete::generate(shells::Zsh, &mut app.clone(), "spikeq", &mut zsh_buf);
    fs::write(zsh_dir.join("_spikeq"), zsh_buf)?;
    println!("cargo:warning=Zsh completions generated successfully");

    // Fish completions
    println!("cargo:warning=Generating Fish completions");
    let mut fish_buf = Vec::new();
    clap_complete::generate(shells::Fish, &mut app.clone(), "spikeq", &mut fish_buf);
    fs::write(fish_dir.join("spikeq.fish"), fish_buf)?;
    println!("cargo:warning=Fish completions generated successfully");

    println!(
        "cargo:warning=Shell completions generated in {}",
        completions_dir.display()
    );

    Ok(())
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/arg.rs");

    // Get the output directory for our generated assets
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => PathBuf::from(outdir),
        None => {
            println!("cargo:warning=Failed to get OUT_DIR");
            return Ok(());
        }
    };

    // Generate manpage
    if let Err(e) = generate_man_page(&outdir) {
        println!("cargo:warning=Failed to generate man page: {}", e);
    }

    // Generate shell completions
    if let Err(e) = generate_shell_completions(outdir.as_path()) {
        println!("cargo:warning=Failed to generate shell completions: {}", e);
    } else {
        // Add further instructions for installation
        println!("cargo:warning=");
        println!("cargo:warning=To install the man page:");
        println!(
            "cargo:warning=  sudo cp {}/man/man1/spikeq.1 /usr/local/share/man/man1/",
            outdir.display()
        );
        println!("cargo:warning=");
        println!("cargo:warning=To install shell completions:");
        println!(
            "cargo:warning=  Bash: cp {}/completions/bash/spikeq.bash ~/.bash_completion",
            outdir.display()
        );
        println!(
            "cargo:warning=  Zsh:  cp {}/completions/zsh/_spikeq ~/.zfunc/",
            outdir.display()
        );
        println!(
            "cargo:warning=  Fish: cp {}/completions/fish/spikeq.fish ~/.config/fish/completions/",
            outdir.display()
        );
    }

    Ok(())
}
