// SPDX-License-Identifier: GPL-3.0-only

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

struct Step {
    name: &'static str,
    args: &'static [&'static str],
}

const STEPS: &[Step] = &[
    Step {
        name: "cargo fmt --check",
        args: &["fmt", "--all", "--", "--check"],
    },
    Step {
        name: "cargo clippy -D warnings",
        args: &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    },
    Step {
        name: "cargo test",
        args: &["test", "--workspace"],
    },
];

const DENY: Step = Step {
    name: "cargo deny check",
    args: &["deny", "check"],
};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("check") => {
            let rest: Vec<String> = args.collect();
            if rest.iter().any(|a| a != "--skip-deny") {
                return usage();
            }
            check(!rest.is_empty())
        }
        _ => usage(),
    }
}

fn usage() -> ExitCode {
    eprintln!("uso: cargo xtask check [--skip-deny]");
    ExitCode::from(2)
}

fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.ancestors().nth(2).map(PathBuf::from);
    root.unwrap_or(manifest)
}

fn cargo() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn deny_installed(root: &Path) -> bool {
    Command::new(cargo())
        .args(["deny", "--version"])
        .current_dir(root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run(step: &Step, root: &Path) -> bool {
    println!("\n==> {}", step.name);
    match Command::new(cargo())
        .args(step.args)
        .current_dir(root)
        .status()
    {
        Ok(status) => status.success(),
        Err(err) => {
            eprintln!("não foi possível executar o cargo: {err}");
            false
        }
    }
}

fn check(skip_deny: bool) -> ExitCode {
    let root = workspace_root();
    let mut results: Vec<(&str, bool)> = Vec::new();

    for step in STEPS {
        results.push((step.name, run(step, &root)));
    }

    if !skip_deny {
        if deny_installed(&root) {
            results.push((DENY.name, run(&DENY, &root)));
        } else {
            eprintln!("\n==> {}", DENY.name);
            eprintln!("cargo-deny não encontrado. Instale com: cargo install cargo-deny --locked");
            results.push((DENY.name, false));
        }
    }

    println!("\nResumo:");
    for (name, ok) in &results {
        println!("  [{}] {name}", if *ok { "ok" } else { "falhou" });
    }

    if results.iter().all(|(_, ok)| *ok) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
