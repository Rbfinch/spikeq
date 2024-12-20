mod arg;
mod iupac;
mod read_regex;

use crate::arg::{Args, Commands}; // Import Args and Commands
use clap::Parser;
use iupac::get_iupac_regexes;
use rand::seq::SliceRandom;
use rand::Rng;
use read_regex::read_base_strings_from_json;
use regex::Regex;
use serde::{Deserialize, Serialize}; // Added Serialize
use std::fs;
use uuid::Uuid;

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

fn generate_sequence(min_length: usize, max_length: usize, regex_patterns: &[Regex]) -> String {
    let chars = ['A', 'C', 'T', 'G'];
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(min_length..=max_length);
    loop {
        let sequence: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        if !regex_patterns.iter().any(|re| re.is_match(&sequence)) {
            return sequence;
        }
    }
}

fn generate_quality_line(length: usize, regex_patterns: &[Regex]) -> String {
    let chars: Vec<char> = r#"!\"\#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#.chars().collect();
    let mut rng = rand::thread_rng();
    loop {
        let line: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        if !regex_patterns.iter().any(|re| re.is_match(&line)) {
            return line;
        }
    }
}

fn insert_patterns(sequence: &mut String, patterns: &[Regex]) {
    let mut rng = rand::thread_rng();
    for pattern in patterns {
        let pos = rng.gen_range(0..=sequence.len());
        sequence.insert_str(pos, pattern.as_str());
    }
}

fn main() {
    let args = Args::parse();

    let mut regex_patterns: Vec<Regex> = vec![];

    if let Some(regex_patterns_file) = &args.regex_patterns {
        let additional_patterns = read_base_strings_from_json(regex_patterns_file)
            .expect("Unable to read forbidden patterns file");
        let iupac_regexes = get_iupac_regexes();
        for pattern in additional_patterns {
            let mut expanded_patterns = vec![pattern];
            for (re, replacements) in &iupac_regexes {
                expanded_patterns = expand_strings(expanded_patterns, re, replacements);
            }
            regex_patterns.extend(
                expanded_patterns
                    .into_iter()
                    .map(|p| Regex::new(&p).expect("Invalid regex pattern")),
            );
        }
    }

    let uuid = Uuid::new_v4().to_string();
    let mut output = String::new();

    let (min_length, max_length) = args.length;

    match &args.command {
        Some(Commands::SpikeSequence {
            num_patterns,
            num_sequences: num_spiked_sequences,
        }) => {
            let mut rng = rand::thread_rng();
            let selected_patterns: Vec<Regex> = regex_patterns
                .choose_multiple(&mut rng, *num_patterns)
                .cloned()
                .collect();

            let mut pattern_counts = vec![0; *num_patterns];

            for i in 0..args.num_sequences {
                let mut sequence = generate_sequence(min_length, max_length, &regex_patterns);
                if i < *num_spiked_sequences {
                    insert_patterns(&mut sequence, &selected_patterns);
                    for (j, pattern) in selected_patterns.iter().enumerate() {
                        if sequence.contains(pattern.as_str()) {
                            pattern_counts[j] += 1;
                        }
                    }
                }
                let quality_line = generate_quality_line(sequence.len(), &regex_patterns);
                output.push_str(&format!(
                    "@SRX22685872.1 A00627:493:HKF5GDSX5:1:1101:15239:1047 length={}\n{}\n+\n{}\n", // dummy header
                    sequence.len(),
                    sequence,
                    quality_line
                ));
            }

            // Ensure the summary writing block is executed
            let summary: Vec<_> = selected_patterns
                .iter()
                .zip(pattern_counts.iter())
                .map(|(pattern, &count)| {
                    serde_json::json!({
                        "Spiked pattern": pattern.as_str(),
                        "Number of insertions of spiked pattern": count
                    })
                })
                .collect();

            let regex_set_name = if let Some(regex_patterns_file) = &args.regex_patterns {
                let file = std::fs::File::open(regex_patterns_file).expect("Unable to open file");
                let reader = std::io::BufReader::new(file);
                let json: serde_json::Value =
                    serde_json::from_reader(reader).expect("Unable to parse JSON");
                json["regexSet"]["regexSetName"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string()
            } else {
                "unknown".to_string()
            };

            let output_json = serde_json::json!({
                "Number of generated FASTQ records": args.num_sequences,
                "Number of spiked patterns": num_patterns,
                "Number of spiked sequences": num_spiked_sequences,
                "Name of generated FASTQ file": uuid,
                "Minimum sequence length": min_length,
                "Maximum sequence length": max_length,
                "Name of the regex set": regex_set_name, // Modified line
                "Summary of spiked patterns": summary
            });

            let summary_json =
                serde_json::to_string_pretty(&output_json).expect("Failed to serialize summary");
            fs::write("inserted.json", summary_json).expect("Unable to write to inserted.json");
        }
        None => {
            for _ in 0..args.num_sequences {
                let sequence = generate_sequence(min_length, max_length, &regex_patterns);
                let quality_line = generate_quality_line(sequence.len(), &regex_patterns);
                output.push_str(&format!(
                    "@{}:{} length={}\n{}\n+\n{}\n",
                    "default_set_name",
                    "default_spiked_pattern",
                    sequence.len(),
                    sequence,
                    quality_line
                ));
            }
        }
    }

    fs::write(format!("{}", uuid), output).expect("Unable to write to file");
}

fn expand_strings(strings: Vec<String>, re: &Regex, replacements: &[&str]) -> Vec<String> {
    let mut result = vec![];

    for s in strings {
        let mut temp = vec![s];
        while re.is_match(&temp[0]) {
            temp = temp
                .into_iter()
                .flat_map(|s| {
                    replacements
                        .iter()
                        .map(move |&replacement| re.replace(&s, replacement).to_string())
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        result.extend(temp);
    }

    result
}
