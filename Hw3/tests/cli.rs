use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn test_no_input_grade() -> TestResult {
   let mut cmd = Command::cargo_bin("grade")?;
   cmd.assert().failure();
    Ok(())

}
#[test]
fn test_grade() -> TestResult {
    let mut cmd = Command::cargo_bin("grade")?;
    cmd.arg("45").assert().success().stdout("Failed with F");
    Ok(())
}

#[test]
fn test_no_input_cf() -> TestResult {
   let mut cmd = Command::cargo_bin("cf")?;
   cmd.assert().failure();
    Ok(())

}
#[test]
fn test_cf() -> TestResult {
    let mut cmd = Command::cargo_bin("cf")?;
    cmd.arg("0").arg("2").arg("1").assert().success()
    .stdout("Fahr Celcius\n0°F -17.8°C\n1°F -17.2°C\n2°F -16.7°C\n");
    Ok(())
}

#[test]
fn test_no_star() -> TestResult {
    let mut cmd = Command::cargo_bin("star1")?;
    cmd.assert().success()
    .stdout("");
    Ok(())
}

#[test]
fn test_with_star() -> TestResult {
    let mut cmd = Command::cargo_bin("star1")?;
    cmd.arg("2").assert().success()
    .stdout("*\n**\n*\n");
    Ok(())
}

#[test]
fn test_no_star2() -> TestResult {
    let mut cmd = Command::cargo_bin("star2")?;
    cmd.assert().success()
    .stdout("");
    Ok(())
}

#[test]
fn test_with_star2() -> TestResult {
    let mut cmd = Command::cargo_bin("star2")?;
    cmd.arg("2").assert().success()
    .stdout(" *\n**\n *\n");
    Ok(())
}