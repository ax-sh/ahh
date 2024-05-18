#[cfg(test)]
mod ahh_integration {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_default_ahh_execution() {
        let mut cmd = Command::cargo_bin("ahh").unwrap();

        cmd.args(&["When did the unix time start", "(Just the year)"])
            .assert()
            .success()
            .stderr("")
            .stdout(predicate::str::contains("1970"));
    }
}
