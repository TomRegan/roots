extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn default_config_path_is_displayed() {
    let mut cmd = Command::cargo_bin("roots").unwrap();
    cmd.arg("config")
        .arg("--path");
    let assert = cmd.assert();
    assert.success()
        .stdout("Default configuration\n")
        .code(0);
}

#[test]
fn default_and_path_flags_conflict() {
    let mut cmd = Command::cargo_bin("roots").unwrap();
    cmd.arg("config")
        .arg("--path")
        .arg("--default");
    let assert = cmd.assert();
    assert.failure()
        .code(1);
}


