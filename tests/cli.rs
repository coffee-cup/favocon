use assert_cmd::prelude::*;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

fn create_icon(path: &Path) -> Result<(), Box<std::error::Error>> {
    let tmp_dir = TempDir::new("out")?;
    let outdir = tmp_dir.path();

    let mut cmd = Command::cargo_bin("favocon")?;

    cmd.arg(path).arg("-o").arg(outdir);
    cmd.assert().success();

    let file = std::fs::File::open(outdir.join("favicon.ico")).unwrap();
    let icon_dir = ico::IconDir::read(file).unwrap();

    assert_eq!(icon_dir.entries().len(), 3);

    Ok(())
}

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
fn invalid_size_icon() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("favocon")?;

    cmd.arg("tests/invalid_icon.png");
    cmd.assert().failure().stderr("Image must be square\n");

    Ok(())
}

#[test]
fn icon_created_from_png() -> Result<(), Box<std::error::Error>> {
    create_icon(Path::new("tests/icon.png"))
}

#[test]
fn icon_created_from_jpg() -> Result<(), Box<std::error::Error>> {
    create_icon(Path::new("tests/icon.jpg"))
}
