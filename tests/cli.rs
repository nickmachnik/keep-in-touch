use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

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

#[test]
fn remove_non_existing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("remove").arg("Daniel");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not in the list"));

    Ok(())
}

#[test]
fn whole_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("add").arg("X").arg("30").arg("now");
    cmd.assert().stderr(predicate::str::contains("Added \"X\""));
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("modify").arg("X").arg("interval").arg("10");
    cmd.assert()
        .stderr(predicate::str::contains("Modified \"X\""));
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("modify").arg("X").arg("last").arg("2015-10-10");
    cmd.assert()
        .stderr(predicate::str::contains("Modified \"X\""));
    let mut cmd = Command::cargo_bin("kit")?;
    cmd.arg("remove").arg("X");
    cmd.assert()
        .stderr(predicate::str::contains("Removed \"X\""));
    Ok(())
}
