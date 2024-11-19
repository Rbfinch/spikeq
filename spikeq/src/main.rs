use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
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
struct Config {
    #[serde(flatten)]
    regex_set: RegexSet,
}

fn generate_sequence(length: usize, forbidden_patterns: &[Regex]) -> String {
    let chars = ['A', 'C', 'T', 'G'];
    let mut rng = rand::thread_rng();
    loop {
        let sequence: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();
        if !forbidden_patterns.iter().any(|re| re.is_match(&sequence)) {
            return sequence;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        std::process::exit(1);
    }
    let config_file = &args[1];

    let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("Unable to parse config file");

    let forbidden_patterns: Vec<Regex> = config
        .regex_set
        .regex
        .iter()
        .map(|pattern| Regex::new(&pattern.regex_string).expect("Invalid regex pattern"))
        .collect();

    let sequence = generate_sequence(150, &forbidden_patterns);
    println!("Generated sequence: {}", sequence);
}
