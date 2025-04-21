use miette::{Diagnostic, SourceSpan};
use std::num::ParseIntError;
use std::path::PathBuf;
use thiserror::Error;

/// The main error type for the SpikeQ application
#[derive(Error, Diagnostic, Debug)]
pub enum SpikeQError {
    #[error("Failed to read file: {path}")]
    #[diagnostic(code(spikeq::io::read_error))]
    FileReadError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Failed to write file: {path}")]
    #[diagnostic(code(spikeq::io::write_error))]
    FileWriteError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("JSON parsing error: {message}")]
    #[diagnostic(code(spikeq::json::parse_error))]
    JsonParseError {
        #[source]
        source: serde_json::Error,
        message: String,
    },

    #[error("Invalid JSON schema: {0}")]
    #[diagnostic(code(spikeq::json::schema_error))]
    JsonSchemaError(String),

    #[error("JSON validation failed: {reason}")]
    #[diagnostic(code(spikeq::json::validation_error))]
    JsonValidationError {
        reason: String,
        #[related]
        errors: Vec<JsonFieldError>,
    },

    #[error("Invalid regex pattern: {pattern}")]
    #[diagnostic(code(spikeq::regex::invalid_pattern))]
    RegexError {
        #[source]
        source: regex::Error,
        pattern: String,
    },

    #[error("Invalid sequence length range: {0}")]
    #[diagnostic(code(spikeq::args::length_range))]
    #[allow(dead_code)]
    InvalidLengthRange(String),

    #[error("Parse integer error: {0}")]
    #[diagnostic(code(spikeq::args::parse_int))]
    ParseIntError(#[from] ParseIntError),

    #[error("No regex patterns available for spiking")]
    #[diagnostic(code(spikeq::spike::no_patterns))]
    NoRegexPatternsError,

    #[error("Requested more patterns ({requested}) than available ({available})")]
    #[diagnostic(code(spikeq::spike::insufficient_patterns))]
    InsufficientPatternsError { requested: usize, available: usize },

    #[error("Failed to generate sequence after {attempts} attempts")]
    #[diagnostic(code(spikeq::sequence::generation_failed))]
    SequenceGenerationError { attempts: usize },

    #[error("{0}")]
    #[diagnostic(code(spikeq::general::operation_failed))]
    GeneralError(String),
}

// Implement From<String> for SpikeQError to allow ? operator with String errors
impl From<String> for SpikeQError {
    fn from(s: String) -> Self {
        SpikeQError::GeneralError(s)
    }
}

// Implement From<&str> for SpikeQError for more ergonomic error creation
impl From<&str> for SpikeQError {
    fn from(s: &str) -> Self {
        SpikeQError::GeneralError(s.to_string())
    }
}

/// Represents a specific error in a JSON field
#[derive(Error, Diagnostic, Debug)]
#[error("Error in field {field_path}: {message}")]
pub struct JsonFieldError {
    field_path: String,
    message: String,
    #[source_code]
    src: Option<String>,
    #[label("this part caused the error")]
    err_span: Option<SourceSpan>,
}

impl JsonFieldError {
    pub fn new(field_path: String, message: String) -> Self {
        Self {
            field_path,
            message,
            src: None,
            err_span: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_source(mut self, source: String, span: (usize, usize)) -> Self {
        self.src = Some(source);
        self.err_span = Some(span.into());
        self
    }
}

/// Type alias for the Result type used throughout SpikeQ
pub type Result<T> = std::result::Result<T, SpikeQError>;
