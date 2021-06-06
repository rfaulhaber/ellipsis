use std::{fs, path::Path, process::Command};

#[test]
fn test() {
    let install_output = Command::new("./target/debug/ellipsis")
        .arg("--config")
        .arg("./tests/ellipsis.yml")
        .arg("install")
        .arg("install_test")
        .output()
        .expect("install command should run successfully");

    println!(
        "stdout: {}",
        String::from_utf8(install_output.stdout).unwrap()
    );
    println!(
        "stderr : {}",
        String::from_utf8(install_output.stderr).unwrap()
    );

    assert!(install_output.status.success(), "process failed",);
    assert!(Path::new("./test.txt").exists());
    assert!(Path::new("./foo").exists());
    assert!(Path::new("./foo").is_dir());
    assert!(fs::metadata("./foo/test.txt").unwrap().is_file());

    fs::remove_file("./foo/test.txt").expect("Could not remove file");

    let link_output = Command::new("./target/debug/ellipsis")
        .arg("--config")
        .arg("./tests/ellipsis.yml")
        .arg("link")
        .arg("link_test")
        .output()
        .expect("link command should run successfully");

    println!("stdout: {}", String::from_utf8(link_output.stdout).unwrap());
    println!(
        "stderr : {}",
        String::from_utf8(link_output.stderr).unwrap()
    );

    assert!(link_output.status.success(), "process failed",);
    assert!(Path::new("./foo/test.txt").exists());

    let link_metadata =
        fs::symlink_metadata("./foo/test.txt").expect("should be able to get metadata for file");
    assert!(link_metadata.file_type().is_symlink());

    let exec_output = Command::new("./target/debug/ellipsis")
        .arg("--config")
        .arg("./tests/ellipsis.yml")
        .arg("exec")
        .arg("exec_test")
        .arg("cmd")
        .output()
        .expect("link command should run successfully");

    println!("stdout: {}", String::from_utf8(exec_output.stdout).unwrap());
    println!(
        "stderr : {}",
        String::from_utf8(exec_output.stderr).unwrap()
    );
}
