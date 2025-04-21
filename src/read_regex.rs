use crate::error::{JsonFieldError, Result, SpikeQError};
use serde_json::Value;
use std::fs::File;
use std::io;
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

pub fn read_base_strings_from_json<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let path_buf = filename.as_ref().to_path_buf();
    let file = File::open(&filename).map_err(|e| SpikeQError::FileReadError {
        source: e,
        path: path_buf.clone(),
    })?;

    let reader = io::BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).map_err(|e| SpikeQError::JsonParseError {
        source: e,
        message: format!("Failed to parse JSON file: {}", path_buf.display()),
    })?;

    let schema: Value = serde_json::from_str(SCHEMA).map_err(|e| {
        SpikeQError::JsonSchemaError(format!("Failed to parse embedded schema: {}", e))
    })?;

    let validator = jsonschema::validator_for(&schema)
        .map_err(|e| SpikeQError::JsonSchemaError(format!("Failed to compile schema: {}", e)))?;

    let mut json_field_errors = Vec::new();
    for error in validator.iter_errors(&json) {
        json_field_errors.push(JsonFieldError::new(
            error.instance_path.to_string(),
            error.to_string(),
        ));
    }

    if !json_field_errors.is_empty() {
        return Err(SpikeQError::JsonValidationError {
            reason: "The provided JSON file does not match the required schema".to_string(),
            errors: json_field_errors,
        });
    }

    let base_strings: Vec<String> = json["regexSet"]["regex"]
        .as_array()
        .ok_or_else(|| {
            SpikeQError::JsonSchemaError(
                "Missing or invalid 'regex' array in JSON file".to_string(),
            )
        })?
        .iter()
        .filter_map(|r| {
            r.get("regexString")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    if base_strings.is_empty() {
        return Err(SpikeQError::JsonSchemaError(
            "No valid regex patterns found in the JSON file".to_string(),
        ));
    }

    Ok(base_strings)
}
