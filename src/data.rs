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
#[derive(Clone)]
pub struct Command {
    pub title: String,
    pub command: String,
}

/// A Job contains a bunch of Commands to run in sequence.
#[derive(Clone)]
pub struct Job {
    pub ecosystem: Ecosystem,
    pub steps: Vec<Command>,
}

lazy_static! {
    pub static ref INSTALL_OTEL_CLI: Command = Command {
        title: "install otel-cli".into(),
        command: "curl -L https://github.com/equinix-labs/otel-cli/releases/download/v0.0.20/otel-cli-0.0.20-Linux-x86_64.tar.gz | sudo tar xvzf - -C /usr/local/bin".into(),
    };
    pub static ref CC_TEST: Job = Job {
        ecosystem: Ecosystem::Cargo,
        steps: vec![
            Command {
                title: "configure traceparent".into(),
                command: "if [ -n \"$HOOK_URL\"]; then curl \"${HOOK_URL}/traceparent/${CIRCLE_WORKFLOW_ID}/${CIRCLE_WORKFLOW_JOB_ID}\" >> \"$BASH_ENV\"; fi".into(),
            },
            INSTALL_OTEL_CLI.clone(),
            Command {
                title: "install protoc".to_string(),
                command: "sudo apt update && sudo apt install -y protobuf-compiler".to_string(),
            },
            // checkout
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

// #[derive(Debug, Serialize)]
// pub struct CciYml {
//     pub version: String,
//     pub jobs: Value,
// }
