use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn test_no_args() -> TestResult {
   let mut cmd = Command::cargo_bin("hw")?;
   cmd.assert().failure();
    Ok(())

}
#[test]
fn test_area() -> TestResult {
    let mut cmd = Command::cargo_bin("hw")?;
    cmd.arg("1").assert().success().stdout("Area is 3.1416");
    Ok(())


}
#[test]
fn test_no_args_temp() -> TestResult {
    let mut cmd = Command::cargo_bin("temperature")?;
    cmd.assert().failure();
     Ok(())
}

#[test]
fn test_temp() -> TestResult {
    let mut cmd = Command::cargo_bin("temperature")?;
    cmd.arg("1").assert().success().stdout("1°F is -17.222223°C");
     Ok(())
}

#[test]
fn test_no_args_temp_reverse() -> TestResult {
    let mut cmd = Command::cargo_bin("temp_reverse")?;
    cmd.assert().failure();
     Ok(())
}

#[test]
fn test_temp_reverse() -> TestResult {
    let mut cmd = Command::cargo_bin("temp_reverse")?;
    cmd.arg("1").assert().success().stdout("1°C is 33.8°F");
     Ok(())
}

#[test]
fn test_no_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_players").assert().success()
    .stdout("Player 1 : N/A\nPlayer 2 : N/A");
     Ok(())
}

#[test]
fn test_onename_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_players").arg("Mike").assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : N/A");
     Ok(())
}

#[test]
fn test_twoname_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_players").arg("Mike").arg("Leo").assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : Leo");
     Ok(())
}

#[test]
fn test_morethantwo_name_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_players").arg("Mike").arg("Leo").arg("Ralph")
    .assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : Leo");
     Ok(())
}