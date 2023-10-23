use assert_cmd::Command;
use float_cmp::approx_eq;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args() -> TestResult {
   let mut cmd = Command::cargo_bin("slope")?;
   cmd.assert().failure();
    Ok(())

}

#[test]
fn slope_test() -> TestResult {
    let mut cmd = Command::cargo_bin("slope")?;
    cmd.arg("10").arg("11").arg("12").arg("13")
    .assert().success()
    .stdout("m = 1");
     Ok(())
}

#[test]
fn test_float() {
assert!(approx_eq!(f64, 1. / 3., 0.3333, epsilon = 0.0001));
}




#[test]
fn test_with_star() -> TestResult {
   let mut cmd = Command::cargo_bin("triangle")?;
   cmd.arg("5").assert().success().stdout("*\n**\n***\n****\n*****\n");
    Ok(())

}