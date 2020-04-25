use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add_existing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("add").arg("Martin").arg("30").arg("now");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already used"));

    Ok(())
}

#[test]
fn add_wrong_date_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("add").arg("Martin").arg("30").arg("20baba");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Parsing the date string failed"));

    Ok(())
}

#[test]
fn add_wrong_interval_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("add").arg("Martin").arg("bubu").arg("now");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Parsing the interval field failed",
    ));

    Ok(())
}
