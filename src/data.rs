use clap::ValueEnum;
use lazy_static::lazy_static;

/// This enum lists all detectable tool/language ecosystems.
#[derive(Debug, Clone, ValueEnum)]
pub enum Ecosystem {
    Bundler,
    Cargo,
    Pip,
}

/// This enum lists all supported CI Pipeline configuration targets.
#[derive(Debug, Clone, ValueEnum)]
pub enum Target {
    CircleCI,
    GitHubActions,
    GitLabCI,
}

/// A shell command to execute somewhere.
pub struct Command {
    title: String,
    command: String,
}

/// A Job contains a bunch of Commands to run in sequence.
pub struct Job {
    ecosystem: Ecosystem,
    steps: Vec<Command>,
}

lazy_static! {
    pub static ref CC_TEST: Job = Job {
        ecosystem: Ecosystem::Cargo,
        steps: vec![
            // checkout
            Command {
                title: "install protoc".to_string(),
                command: "sudo apt install -y protobuf-compiler".to_string(),
            },
            // restore cache
            Command {
                title: "cargo check".to_string(),
                command: "cargo check --workspace".to_string(),
            },
            Command {
                title: "cargo test".to_string(),
                command: "cargo test --workspace --doc".to_string(),
            },
            Command {
                title: "cargo build".to_string(),
                command: "cargo build --workspace --release".to_string(),
            },
            // export artifacts
            // save cache
        ],
    };
}
