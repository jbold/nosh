use crate::content_types::StrictWarning;
use crate::schema::ValidationError;
use serde::Serialize;

#[derive(Serialize)]
pub struct FileResult {
    pub path: String,
    pub valid: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<WarningEntry>,
}

#[derive(Serialize)]
pub struct ErrorEntry {
    pub path: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WarningEntry {
    pub field: String,
    pub message: String,
}

pub fn format_human(results: &[FileResult], quiet: bool) -> String {
    let mut out = String::new();
    for r in results {
        if r.valid && r.warnings.is_empty() {
            if !quiet {
                let nosh_type = ""; // simplified — just show valid
                out.push_str(&format!("  Valid: {}\n", r.path));
                let _ = nosh_type;
            }
        } else if r.valid && !r.warnings.is_empty() {
            out.push_str(&format!("  Valid (with warnings): {}\n", r.path));
            for w in &r.warnings {
                out.push_str(&format!("    warning [{}]: {}\n", w.field, w.message));
            }
        } else {
            out.push_str(&format!("  Invalid: {}\n", r.path));
            for e in &r.errors {
                out.push_str(&format!("    error [{}]: {}\n", e.path, e.message));
            }
            for w in &r.warnings {
                out.push_str(&format!("    warning [{}]: {}\n", w.field, w.message));
            }
        }
    }
    out
}

pub fn format_json(results: &[FileResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string())
}

pub fn build_result(
    path: String,
    errors: Vec<ValidationError>,
    warnings: Vec<String>,
    strict_warnings: Vec<StrictWarning>,
) -> FileResult {
    let valid = errors.is_empty();
    let mut all_warnings: Vec<WarningEntry> = warnings
        .into_iter()
        .map(|msg| WarningEntry {
            field: String::new(),
            message: msg,
        })
        .collect();
    all_warnings.extend(strict_warnings.into_iter().map(|sw| WarningEntry {
        field: sw.field,
        message: sw.message,
    }));

    FileResult {
        path,
        valid,
        errors: errors
            .into_iter()
            .map(|e| ErrorEntry {
                path: e.path,
                message: e.message,
            })
            .collect(),
        warnings: all_warnings,
    }
}
