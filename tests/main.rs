use assert_cmd::Command;

#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chunked_transfer_cli")?;
    cmd
        .arg("decode")
        .write_stdin("3\r\nhel\r\nb\r\nlo world!!!\r\n0\r\n\r\n");
    cmd.assert()
        .success()
        .stdout("hello world!!!");

    Ok(())
}

#[test]
fn encode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chunked_transfer_cli")?;
    cmd
        .arg("encode")
        .arg("--chunk-size").arg("5")
        .write_stdin("hello world");
    cmd.assert()
        .success()
        .stdout("5\r\nhello\r\n5\r\n worl\r\n1\r\nd\r\n0\r\n\r\n");

    Ok(())
}
