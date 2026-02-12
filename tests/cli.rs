use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: roll"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("roll 0.1.0"));
}

#[test]
fn test_single_die() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("1d20")
        .assert()
        .success()
        .stdout(predicate::str::contains("d20"))
        // Check for table structure roughly
        .stdout(predicate::str::contains("| Die"));
}

#[test]
fn test_multiple_dice() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("2d6")
        .arg("1d10")
        .assert()
        .success()
        .stdout(predicate::str::contains("d6"))
        .stdout(predicate::str::contains("d10"))
        .stdout(predicate::str::contains("Total"));
}

#[test]
fn test_invalid_arg() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: Failed to parse dice expression",
        ));
}

#[test]
fn test_partial_valid_arg() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("1d20extra")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid dice format"));
}

#[test]
fn test_zero_sides() {
    let mut cmd = Command::cargo_bin("roll").unwrap();
    cmd.arg("2d0")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: Dice cannot have 0 sides."));
}
