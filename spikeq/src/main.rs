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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <config_file> [-n <number_of_sequences>]",
            args[0]
        );
        std::process::exit(1);
    }
    let config_file = &args[1];
    let num_sequences = if args.len() == 4 && args[2] == "-n" {
        args[3]
            .parse::<usize>()
            .expect("Invalid number of sequences")
    } else {
        1
    };

    let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("Unable to parse config file");

    let forbidden_patterns: Vec<Regex> = config
        .regex_set
        .regex
        .iter()
        .map(|pattern| Regex::new(&pattern.regex_string).expect("Invalid regex pattern"))
        .collect();

    for _ in 0..num_sequences {
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
