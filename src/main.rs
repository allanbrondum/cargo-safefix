use std::{env, process::Command};

pub fn main() {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);

    let args_full: Vec<_> = env::args_os().collect();
    let mut args_split = args_full.split(|arg| arg == "--");

    let args_safefix: Vec<_> = args_split.next().unwrap_or_default().iter().collect();
    let mut dry = false;
    let args_cargo: Vec<_> = args_safefix
        .iter()
        .filter(|&&arg| {
            dry |= arg == "--dry";
            arg != "--dry"
        })
        .collect();

    if dry {
        command.args(["clippy", "--frozen"]);
    } else {
        command.args([
            "clippy",
            "--fix",
            "--allow-dirty",
            "--allow-staged",
            "--frozen",
            "--all-features",
        ]);
    }

    command.args(args_cargo.iter().skip(2));

    command.arg("--");

    command.args(["-A", "clippy::all"]);
    const ALL_GROUPS: &[&str] = &[
        "deprecated-safe",
        "future-incompatible",
        "keyword-idents",
        "let-underscore",
        "nonstandard-style",
        "refining-impl-trait",
        "rust-2018-compatibility",
        "rust-2018-idioms",
        "rust-2021-compatibility",
        "rust-2024-compatibility",
        "unused",
    ];
    for fix in ALL_GROUPS {
        command.args(["-A", fix]);
    }

    const SAFE_TO_FIX: &[&str] = &["unused_imports", "clippy::needless_borrow"];
    for fix in SAFE_TO_FIX {
        command.args(["-W", fix]);
    }

    for args in args_split {
        command.args(args);
    }

    println!("command {:?}", command);

    let results = command
        .status()
        .expect("Failed to invoke cargo! Make sure it's in your $PATH");
    std::process::exit(results.code().unwrap());
}
