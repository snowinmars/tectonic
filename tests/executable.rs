// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate tempdir;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str;
use tempdir::TempDir;

fn run_tectonic(cwd: &Path, args: &[&str]) -> Output {
    let tectonic = cargo_dir()
        .join("tectonic")
        .with_extension(env::consts::EXE_EXTENSION);

    match fs::metadata(&tectonic) {
        Ok(_) => {}
        Err(_) => {
            panic!("tectonic binary not found at {:?}. Do you need to run `cargo build`?",
                   tectonic)
        }
    }
    println!("using tectonic binary at {:?}", tectonic);
    println!("using cwd {:?}", cwd);

    let mut command = Command::new(tectonic);
    command.args(args);
    command.current_dir(cwd);
    println!("running {:?}", command);

    return command.output().expect("tectonic failed to start");
}

fn setup_and_copy_files(files: &[&str]) -> TempDir {
    let tempdir = TempDir::new("tectonic_executable_test").unwrap();
    let executable_test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests/executable");

    for file in files {
        // Create parent directories, if the file is not at the root of `tests/executable/`
        let file_path = PathBuf::from(file);
        let parent_dir = file_path.parent().unwrap();
        let mut dirbuilder = fs::DirBuilder::new();
        dirbuilder.recursive(true);
        dirbuilder.create(tempdir.path().join(parent_dir)).unwrap();

        fs::copy(executable_test_dir.join(file), tempdir.path().join(file)).unwrap();
    }

    return tempdir;
}

// Duplicated from Cargo's own testing code:
// https://github.com/rust-lang/cargo/blob/19fdb308/tests/cargotest/support/mod.rs#L305-L318
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            env::current_exe()
                .ok()
                .map(|mut path| {
                         path.pop();
                         if path.ends_with("deps") {
                             path.pop();
                         }
                         path
                     })
        })
        .unwrap_or_else(|| panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test"))
}

fn success_or_panic(output: Output) {
    if output.status.success() {
        println!("status: {}", output.status);
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!("Command exited badly:\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
               output.status,
               String::from_utf8_lossy(&output.stdout),
               String::from_utf8_lossy(&output.stderr));
    }
}

/* Keep tests alphabetized */

#[test]
fn help_flag() {
    let tempdir = setup_and_copy_files(&[]);

    let output = run_tectonic(tempdir.path(), &["-h"]);
    success_or_panic(output);
}

#[test]
#[ignore] // FIXME: GitHub #31
fn relative_include() {
    let tempdir = setup_and_copy_files(&["subdirectory/relative_include.tex",
                                         "subdirectory/content/1.tex"]);

    let output = run_tectonic(tempdir.path(),
                              &["--format=plain.fmt.gz", "subdirectory/relative_include.tex"]);
    success_or_panic(output);
}

// Regression #36
#[test]
fn test_space() {
    let tempdir = setup_and_copy_files(&["test space.tex"]);

    let output = run_tectonic(tempdir.path(), &["--format=plain.fmt.gz", "test space.tex"]);
    success_or_panic(output);
}