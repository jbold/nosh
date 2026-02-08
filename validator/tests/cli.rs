use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::path::Path;

fn nosh_cmd() -> assert_cmd::Command {
    cargo_bin_cmd!("nosh")
}

// ── Valid fixtures ────────────────────────────────────────────

#[test]
fn valid_article() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/article.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_tutorial() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/tutorial.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_api_reference() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/api-reference.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_product() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/product.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_recipe() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/recipe.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_faq() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/faq.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_changelog() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/changelog.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_dataset() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/dataset.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_event() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/event.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_profile() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid/profile.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid"));
}

#[test]
fn valid_directory_scan() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/valid"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid").count(10));
}

#[test]
fn valid_quiet_mode() {
    nosh_cmd()
        .args(["validate", "--quiet", "tests/fixtures/valid/article.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn valid_json_output() {
    nosh_cmd()
        .args(["validate", "--json", "tests/fixtures/valid/article.nosh"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""valid": true"#));
}

// ── Invalid fixtures ──────────────────────────────────────────

#[test]
fn invalid_missing_nosh_field() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/missing_nosh_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_missing_type_field() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/missing_type_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_missing_title_field() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/missing_title_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_missing_content_field() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/missing_content_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_missing_body_in_content() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/missing_body_in_content.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_empty_title() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/empty_title.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_bad_version_format() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/bad_version_format.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_additional_top_level_property() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/additional_top_level_property.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_bad_language_tag() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/bad_language_tag.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_wrong_body_type() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/wrong_body_type.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_tutorial_extra_content_field() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/tutorial_extra_content_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn invalid_not_json() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/invalid/not_json.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Invalid"));
}

// ── Edge cases ────────────────────────────────────────────────

#[test]
fn nonexistent_path() {
    nosh_cmd()
        .args(["validate", "tests/fixtures/does_not_exist.nosh"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn empty_directory() {
    let dir = Path::new("tests/fixtures/empty_dir");
    std::fs::create_dir_all(dir).ok();
    nosh_cmd()
        .args(["validate", "tests/fixtures/empty_dir"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("No .nosh files found"));
    std::fs::remove_dir(dir).ok();
}

#[test]
fn strict_mode_valid_tutorial() {
    nosh_cmd()
        .args(["validate", "--strict", "tests/fixtures/valid/tutorial.nosh"])
        .assert()
        .success();
}

#[test]
fn json_output_invalid_file() {
    nosh_cmd()
        .args(["validate", "--json", "tests/fixtures/invalid/missing_nosh_field.nosh"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(r#""valid": false"#));
}
