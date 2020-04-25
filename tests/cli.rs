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
