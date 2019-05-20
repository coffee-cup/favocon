use assert_cmd::prelude::*;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn icon_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("favocon")?;

    cmd.arg("foo.png");
    cmd.assert()
        .failure()
        .stderr("No such file or directory (os error 2)\n");

    Ok(())
}

#[test]
fn icon_exists() -> Result<(), Box<std::error::Error>> {
    let tmp_dir = TempDir::new("out")?;
    let outdir = tmp_dir.path();

    let mut cmd = Command::cargo_bin("favocon")?;

    cmd.arg("tests/icon.png").arg("-o").arg(outdir);
    cmd.assert().success();

    let file = std::fs::File::open(outdir.join("favicon.ico")).unwrap();
    let icon_dir = ico::IconDir::read(file).unwrap();

    assert_eq!(icon_dir.entries().len(), 3);

    Ok(())
}
