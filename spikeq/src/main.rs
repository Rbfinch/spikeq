mod arg;

use crate::arg::{Args, Commands}; // Import Args and Commands
use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize}; // Added Serialize
use std::fs;

#[derive(Deserialize, Clone, Serialize)] // Added Serialize derive
#[allow(dead_code)]
struct RegexPattern {
    regex_name: String,
    regex_string: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct RegexSet {
    regex_set_name: String,
    regex: Vec<RegexPattern>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Config {
    regex_set: RegexSet,
}

fn generate_sequence(min_length: usize, max_length: usize, forbidden_patterns: &[Regex]) -> String {
    let chars = ['A', 'C', 'T', 'G'];
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(min_length..=max_length);
    loop {
        let sequence: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        if !forbidden_patterns.iter().any(|re| re.is_match(&sequence)) {
            return sequence;
        }
    }
}

fn generate_quality_line(length: usize, forbidden_patterns: &[Regex]) -> String {
    let chars: Vec<char> = r#"!\"\#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#.chars().collect();
    let mut rng = rand::thread_rng();
    loop {
        let line: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        if !forbidden_patterns.iter().any(|re| re.is_match(&line)) {
            return line;
        }
    }
}

fn insert_patterns(sequence: &mut String, patterns: &[RegexPattern]) {
    let mut rng = rand::thread_rng();
    for pattern in patterns {
        let pos = rng.gen_range(0..=sequence.len());
        sequence.insert_str(pos, &pattern.regex_string);
    }
}

fn main() {
    let args = Args::parse();

    let config_data = fs::read_to_string(&args.config).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("Unable to parse config file");

    let forbidden_patterns: Vec<Regex> = config
        .regex_set
        .regex
        .iter()
        .map(|pattern| Regex::new(&pattern.regex_string).expect("Invalid regex pattern"))
        .collect();

    match &args.command {
        Some(Commands::SpikeSequence {
            num_patterns,
            num_sequences,
        }) => {
            let mut rng = rand::thread_rng();
            let selected_patterns: Vec<RegexPattern> = config
                .regex_set
                .regex
                .choose_multiple(&mut rng, *num_patterns)
                .cloned()
                .collect();

            let mut pattern_counts = vec![0; *num_patterns];

            for i in 0..args.num_sequences {
                let mut sequence = generate_sequence(100, 600, &forbidden_patterns);
                if i < *num_sequences {
                    insert_patterns(&mut sequence, &selected_patterns);
                    for (j, pattern) in selected_patterns.iter().enumerate() {
                        if sequence.contains(&pattern.regex_string) {
                            pattern_counts[j] += 1;
                        }
                    }
                }
                let quality_line = generate_quality_line(sequence.len(), &forbidden_patterns);
                println!(
                    "@{}:{} length={}",
                    config.regex_set.regex_set_name,
                    config.regex_set.regex[0].regex_name,
                    sequence.len()
                );
                println!("{}", sequence);
                println!("+");
                println!("{}", quality_line);
            }

            // Ensure the summary writing block is executed
            let summary: Vec<_> = selected_patterns
                .iter()
                .zip(pattern_counts.iter())
                .map(|(pattern, &count)| {
                    serde_json::json!({
                        "pattern_name": pattern.regex_name,
                        "inserted_count": count
                    })
                })
                .collect();

            let summary_json =
                serde_json::to_string_pretty(&summary).expect("Failed to serialize summary");
            fs::write("inserted.json", summary_json).expect("Unable to write to inserted.json");
        }
        None => {
            for _ in 0..args.num_sequences {
                let sequence = generate_sequence(100, 600, &forbidden_patterns);
                let quality_line = generate_quality_line(sequence.len(), &forbidden_patterns);
                println!(
                    "@{}:{} length={}",
                    config.regex_set.regex_set_name,
                    config.regex_set.regex[0].regex_name,
                    sequence.len()
                );
                println!("{}", sequence);
                println!("+");
                println!("{}", quality_line);
            }
        }
    }
}
