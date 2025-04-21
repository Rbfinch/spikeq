mod arg;
mod error;
mod iupac;
mod read_regex;

use crate::arg::{Args, Commands};
use crate::error::{Result, SpikeQError};
use clap::Parser;
use colored::Colorize;
use iupac::get_iupac_regexes;
use miette::Result as MietteResult;
use rand::prelude::IndexedRandom;
use rand::Rng;
use read_regex::read_base_strings_from_json;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
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

fn generate_sequence(
    min_length: usize,
    max_length: usize,
    regex_patterns: &[Regex],
) -> Result<String> {
    const MAX_ATTEMPTS: usize = 1000;
    let chars = ['A', 'C', 'T', 'G'];
    let mut rng = rand::rng();
    let length = rng.random_range(min_length..=max_length);

    for attempt in 0..MAX_ATTEMPTS {
        let sequence: String = (0..length)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect();

        if !regex_patterns.iter().any(|re| re.is_match(&sequence)) {
            return Ok(sequence);
        }

        if attempt == MAX_ATTEMPTS - 1 {
            return Err(SpikeQError::SequenceGenerationError {
                attempts: MAX_ATTEMPTS,
            });
        }
    }

    // This should be unreachable due to the error in the loop
    Err(SpikeQError::GeneralError(
        "Failed to generate sequence".to_string(),
    ))
}

fn generate_quality_line(length: usize, regex_patterns: &[Regex]) -> Result<String> {
    const MAX_ATTEMPTS: usize = 1000;
    let chars: Vec<char> = r#"!\"\#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#.chars().collect();
    let mut rng = rand::rng();

    for attempt in 0..MAX_ATTEMPTS {
        let line: String = (0..length)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect();

        if !regex_patterns.iter().any(|re| re.is_match(&line)) {
            return Ok(line);
        }

        if attempt == MAX_ATTEMPTS - 1 {
            return Err(SpikeQError::SequenceGenerationError {
                attempts: MAX_ATTEMPTS,
            });
        }
    }

    // This should be unreachable due to the error in the loop
    Err(SpikeQError::GeneralError(
        "Failed to generate quality line".to_string(),
    ))
}

fn insert_patterns(sequence: &mut String, patterns: &[Regex]) {
    let mut rng = rand::rng();
    for pattern in patterns {
        let pos = rng.random_range(0..=sequence.len());
        // We can't directly get the pattern string from Regex, so we use an empty string
        // The actual pattern text would ideally come from the original pattern string
        sequence.insert_str(pos, pattern.as_str());
    }
}

fn main() -> MietteResult<()> {
    let args = Args::parse();

    let mut regex_patterns: Vec<Regex> = vec![];

    if let Some(regex_patterns_file) = &args.regex_patterns {
        let _regex_file_path = Path::new(regex_patterns_file);
        match read_base_strings_from_json(regex_patterns_file) {
            Ok(additional_patterns) => {
                let iupac_regexes = get_iupac_regexes();
                for pattern in additional_patterns {
                    let mut expanded_patterns = vec![pattern.clone()];
                    for (re, replacements) in &iupac_regexes {
                        expanded_patterns = expand_strings(expanded_patterns, re, replacements);
                    }
                    for expanded_pattern in expanded_patterns {
                        match Regex::new(&expanded_pattern) {
                            Ok(regex) => regex_patterns.push(regex),
                            Err(e) => {
                                return Err(SpikeQError::RegexError {
                                    source: e,
                                    pattern: expanded_pattern,
                                }
                                .into());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e.into());
            }
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
            if regex_patterns.is_empty() {
                return Err(SpikeQError::NoRegexPatternsError.into());
            }

            if *num_patterns > regex_patterns.len() {
                return Err(SpikeQError::InsufficientPatternsError {
                    requested: *num_patterns,
                    available: regex_patterns.len(),
                }
                .into());
            }

            let mut rng = rand::rng();
            let selected_patterns: Vec<Regex> = regex_patterns
                .choose_multiple(&mut rng, *num_patterns)
                .cloned()
                .collect();

            let mut pattern_counts = vec![0; *num_patterns];

            for i in 0..args.num_sequences {
                let mut sequence = generate_sequence(min_length, max_length, &regex_patterns)?;
                if i < *num_spiked_sequences {
                    insert_patterns(&mut sequence, &selected_patterns);
                    for (j, pattern) in selected_patterns.iter().enumerate() {
                        if sequence.contains(pattern.as_str()) {
                            pattern_counts[j] += 1;
                        }
                    }
                }
                let quality_line = generate_quality_line(sequence.len(), &regex_patterns)?;
                output.push_str(&format!(
                    "@SRX22685872.1 A00627:493:HKF5GDSX5:1:1101:15239:1047 length={}\n{}\n+\n{}\n",
                    sequence.len(),
                    sequence,
                    quality_line
                ));
            }

            // Get regex set name
            let regex_set_name = if let Some(regex_patterns_file) = &args.regex_patterns {
                extract_regex_set_name(regex_patterns_file)?
            } else {
                "unknown".to_string()
            };

            // Create summary report
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

            let output_json = serde_json::json!({
                "Number of generated FASTQ records": args.num_sequences,
                "Number of spiked patterns": num_patterns,
                "Number of spiked sequences": num_spiked_sequences,
                "Name of generated FASTQ file": uuid,
                "Minimum sequence length": min_length,
                "Maximum sequence length": max_length,
                "Name of the regex set": regex_set_name,
                "Summary of spiked patterns": summary
            });

            let summary_json = serde_json::to_string_pretty(&output_json).map_err(|e| {
                SpikeQError::JsonParseError {
                    source: e,
                    message: "Failed to serialize summary output".to_string(),
                }
            })?;

            fs::write("inserted.json", summary_json).map_err(|e| SpikeQError::FileWriteError {
                source: e,
                path: Path::new("inserted.json").to_path_buf(),
            })?;
        }
        None => {
            for _ in 0..args.num_sequences {
                let sequence = generate_sequence(min_length, max_length, &regex_patterns)?;
                let quality_line = generate_quality_line(sequence.len(), &regex_patterns)?;
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

    fs::write(&uuid, output).map_err(|e| SpikeQError::FileWriteError {
        source: e,
        path: Path::new(&uuid).to_path_buf(),
    })?;

    println!(
        "{}",
        format!("Successfully created FASTQ file: {}", uuid).green()
    );

    Ok(())
}

fn extract_regex_set_name(file_path: &str) -> Result<String> {
    let file = fs::File::open(file_path).map_err(|e| SpikeQError::FileReadError {
        source: e,
        path: Path::new(file_path).to_path_buf(),
    })?;

    let reader = std::io::BufReader::new(file);
    let json: serde_json::Value =
        serde_json::from_reader(reader).map_err(|e| SpikeQError::JsonParseError {
            source: e,
            message: format!("Failed to parse JSON file: {}", file_path),
        })?;

    json["regexSet"]["regexSetName"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| {
            SpikeQError::JsonSchemaError("Missing 'regexSetName' field in JSON file".to_string())
        })
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
