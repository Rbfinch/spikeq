use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

static SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "spikeq",
    "version": 1,
    "type": "object",
    "properties": {
        "regexSet": {
            "type": "object",
            "properties": {
                "regexSetName": {
                    "type": "string"
                },
                "regex": {
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "properties": {
                            "regexName": {
                                "type": "string"
                            },
                            "regexString": {
                                "type": "string"
                            }
                        },
                        "required": [
                            "regexName",
                            "regexString"
                        ]
                    }
                }
            },
            "required": [
                "regexSetName",
                "regex"
            ]
        }
    },
    "required": [
        "regexSet"
    ]
}
"#;

pub fn read_base_strings_from_json<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader)?;

    let schema: Value = serde_json::from_str(SCHEMA).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse embedded schema: {}", e),
        )
    })?;

    let validator = jsonschema::validator_for(&schema).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to compile schema: {}", e),
        )
    })?;

    let mut error_messages = Vec::new();
    for error in validator.iter_errors(&json) {
        error_messages.push(format!("Error: {error}\nLocation: {}", error.instance_path));
    }

    if !error_messages.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("JSON validation errors: {:?}", error_messages),
        ));
    }

    let base_strings: Vec<String> = json["regexSet"]["regex"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON structure"))?
        .iter()
        .filter_map(|r| {
            r.get("regexString")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    Ok(base_strings)
}
