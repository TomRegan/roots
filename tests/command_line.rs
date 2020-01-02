extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn default_config_path_is_displayed() {
    let mut cmd = Command::cargo_bin("roots").unwrap();
    cmd.arg("config").arg("--path");
    let assert = cmd.assert();
    assert.success().stdout("Default configuration\n").code(0);
}

#[test]
fn default_and_path_flags_conflict() {
    let mut cmd = Command::cargo_bin("roots").unwrap();
    cmd.arg("config").arg("--path").arg("--default");
    let assert = cmd.assert();
    assert.failure().code(1);
}

#[test]
fn fields_handles_no_database() {
    let mut cmd = Command::cargo_bin("roots").unwrap();
    cmd.arg("fields");
    let assert = cmd.assert();
    assert.success().stdout("No available fields, is roots initialised?\n").code(0);
}

#[test]
fn list_handles_no_database() {
    let assert = Command::cargo_bin("roots")
        .unwrap()
        .arg("list")
        .assert();
    assert.success().stdout("No titles to list, is roots initialised?\n").code(0);

}