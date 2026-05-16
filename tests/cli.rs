use assert_cmd::Command;
use predicates::prelude::*;

fn flit() -> Command {
    Command::cargo_bin("flit").unwrap()
}

#[test]
fn help_exits_zero() {
    flit().arg("--help").assert().success();
}

#[test]
fn version_exits_zero() {
    flit().arg("--version").assert().success().stdout(predicate::str::contains("0.1"));
}

#[test]
fn list_exits_zero() {
    flit().arg("list").assert().success();
}

#[test]
fn stats_all_exits_zero() {
    flit().args(["stats", "all"]).assert().success();
}

#[test]
fn stats_today_exits_zero() {
    flit().args(["stats", "today"]).assert().success();
}

#[test]
fn read_missing_file_fails() {
    flit()
        .args(["read", "/tmp/does_not_exist_flit_test.txt"])
        .assert()
        .failure();
}
