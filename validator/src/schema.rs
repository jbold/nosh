use jsonschema::Validator;
use serde_json::Value;

static NOSH_SCHEMA_STR: &str = include_str!("../../spec/nosh.schema.json");

pub struct ValidationError {
    pub path: String,
    pub message: String,
}

pub fn compile_schema() -> Validator {
    let schema: Value = serde_json::from_str(NOSH_SCHEMA_STR)
        .expect("embedded nosh.schema.json is invalid JSON");
    Validator::new(&schema).expect("embedded nosh.schema.json is an invalid JSON Schema")
}

pub fn validate(validator: &Validator, doc: &Value) -> Vec<ValidationError> {
    validator
        .iter_errors(doc)
        .map(|err| ValidationError {
            path: err.instance_path.to_string(),
            message: err.to_string(),
        })
        .collect()
}

pub fn check_file_size(len: u64) -> Option<String> {
    if len > 1_048_576 {
        Some(format!(
            "File size ({len} bytes) exceeds recommended maximum of 1 MB"
        ))
    } else {
        None
    }
}

pub fn check_version(doc: &Value) -> Option<String> {
    if let Some(version) = doc.get("nosh").and_then(|v| v.as_str()) {
        if version != "1.0" {
            return Some(format!(
                "File targets nosh version \"{version}\"; this validator supports 1.0"
            ));
        }
    }
    None
}
