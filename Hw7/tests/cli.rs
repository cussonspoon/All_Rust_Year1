use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_vecsort() -> TestResult {
    let mut cmd = Command::cargo_bin("vecsort")?;
    cmd.arg("0").arg("2").arg("1").arg("20").arg("10").assert().success()
    .stdout("Ascending order: [0, 1, 2, 10, 20]\nDescending order: [20, 10, 2, 1, 0]\n");
    Ok(())
}

#[test]
fn test_loopsort() -> TestResult {
    let mut cmd = Command::cargo_bin("loopsort")?;
    cmd.arg("0").arg("2").arg("1").arg("20").arg("10").assert().success()
    .stdout("Ascending order: [0, 1, 2, 10, 20]\nDescending order: [20, 10, 2, 1, 0]\n");
    Ok(())
}

#[test]
fn test_vecsortpoint() -> TestResult {
    let mut cmd = Command::cargo_bin("vecsortpoint")?;
    cmd.arg("1").arg("5").arg("1").arg("2").arg("7").arg("3").arg("999").assert().success()
    .stdout("Sorted points in ascending order by x and y: [(1, 2), (1, 5), (7, 3)]\nSorted points in descending order by x and y: [(7, 3), (1, 5), (1, 2)]\n");
    Ok(())
}

#[test]
fn test_loopsortpoint() -> TestResult {
    let mut cmd = Command::cargo_bin("loopsortpoint")?;
    cmd.arg("1").arg("5").arg("1").arg("2").arg("7").arg("3").arg("999").assert().success()
    .stdout("Sorted points in ascending order by x and y: [(1, 2), (1, 5), (7, 3)]\nSorted points in descending order by x and y: [(7, 3), (1, 5), (1, 2)]\n");
    Ok(())
}