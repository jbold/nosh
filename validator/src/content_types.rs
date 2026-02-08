use serde_json::Value;

pub struct StrictWarning {
    pub field: String,
    pub message: String,
}

/// In --strict mode, check that content-type-specific fields are present.
/// These are warnings, not errors — the fields are optional per the schema
/// but expected for a complete document of that type.
pub fn strict_validate(doc: &Value) -> Vec<StrictWarning> {
    let content_type = match doc.get("type").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => return vec![],
    };
    let content = match doc.get("content") {
        Some(c) => c,
        None => return vec![],
    };

    match content_type {
        "tutorial" => check_fields(content, &[
            ("steps", "tutorials are expected to have steps"),
            ("prerequisites", "tutorials typically list prerequisites"),
            ("duration", "tutorials typically specify a duration"),
        ]),
        "api-reference" => check_fields(content, &[
            ("endpoints", "API references are expected to have endpoints"),
            ("base_url", "API references typically specify a base_url"),
        ]),
        "product" => check_fields(content, &[
            ("price", "products typically specify a price"),
            ("features", "products typically list features"),
        ]),
        "recipe" => check_fields(content, &[
            ("ingredients", "recipes are expected to have ingredients"),
            ("steps", "recipes are expected to have steps"),
            ("servings", "recipes typically specify servings"),
        ]),
        "faq" => check_fields(content, &[
            ("questions", "FAQs are expected to have questions"),
        ]),
        "changelog" => check_fields(content, &[
            ("entries", "changelogs are expected to have entries"),
        ]),
        "dataset" => check_fields(content, &[
            ("format", "datasets typically specify a format"),
            ("fields", "datasets typically describe their fields"),
        ]),
        "event" => check_fields(content, &[
            ("date", "events typically specify a date"),
            ("location", "events typically specify a location"),
        ]),
        "profile" => check_fields(content, &[
            ("name", "profiles typically include a name"),
        ]),
        "article" => vec![],
        _ => vec![StrictWarning {
            field: "type".to_string(),
            message: format!(
                "Unknown content type \"{content_type}\". Agents will treat this as \"article\"."
            ),
        }],
    }
}

fn check_fields(content: &Value, expected: &[(&str, &str)]) -> Vec<StrictWarning> {
    expected
        .iter()
        .filter(|(field, _)| content.get(*field).is_none())
        .map(|(field, msg)| StrictWarning {
            field: format!("content.{field}"),
            message: msg.to_string(),
        })
        .collect()
}
