use assert_cmd::prelude::*;
use std::path::Path;
use std::process::Command;

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
    let outdir = Path::new("tests/out");

    let mut cmd = Command::cargo_bin("favocon")?;

    cmd.arg("tests/icon.png").arg("-o").arg(outdir);
    cmd.assert().success();

    let file = std::fs::File::open(outdir.join("favicon.ico")).unwrap();
    let icon_dir = ico::IconDir::read(file).unwrap();

    assert_eq!(icon_dir.entries().len(), 3);

    std::fs::remove_dir_all(outdir).unwrap();

    Ok(())
}
