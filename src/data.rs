use std::fmt::Display;

use clap::ValueEnum;
use lazy_static::lazy_static;

/// This enum lists all detectable tool/language ecosystems.
#[derive(Debug, Clone, ValueEnum)]
pub enum Ecosystem {
    Bundler,
    Cargo,
    Pip,
}

impl Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
pub enum Command {
    Checkout,
    RestoreCache {
        key_files: Vec<String>,
    },
    StoreCache {
        key_files: Vec<String>,
        data_paths: Vec<String>,
    },
    Shell {
        title: String,
        command: String,
    },
}

pub struct CommandBuilder;

impl CommandBuilder {
    pub fn shell(title: &str, command: &str) -> Command {
        Command::Shell {
            title: title.into(),
            command: command.into(),
        }
    }
}

/// A Job contains a bunch of Commands to run in sequence.
#[derive(Clone)]
pub struct Job {
    pub ecosystem: Ecosystem,
    pub steps: Vec<Command>,
}

lazy_static! {
    pub static ref INSTALL_OTEL_CLI: Command = Command::Shell {
        title: "install otel-cli".into(),
        command: "curl -L https://github.com/equinix-labs/otel-cli/releases/download/v0.0.20/otel-cli-0.0.20-Linux-x86_64.tar.gz | sudo tar xvzf - -C /usr/local/bin".into(),
    };
    pub static ref CC_TEST: Job = Job {
        ecosystem: Ecosystem::Cargo,
        steps: vec![
            Command::Shell {
                title: "configure traceparent".into(),
                command: "if [ -n \"$HOOK_URL\"]; then curl \"${HOOK_URL}/traceparent/${CIRCLE_WORKFLOW_ID}/${CIRCLE_WORKFLOW_JOB_ID}\" >> \"$BASH_ENV\"; fi".into(),
            },
            INSTALL_OTEL_CLI.clone(),
            CommandBuilder::shell(
                "install protoc",
                "sudo apt update && sudo apt install -y protobuf-compiler"
            ),
            // checkout
            Command::Checkout,
            // restore cache
            Command::RestoreCache { key_files: vec!["Cargo.lock".to_string()]},
            Command::Shell {
                title: "cargo check".to_string(),
                command: "cargo check --workspace".to_string(),
            },
            Command::Shell {
                title: "cargo test".to_string(),
                command: "cargo test --workspace --doc".to_string(),
            },
            Command::Shell {
                title: "cargo build".to_string(),
                command: "cargo build --workspace --release".to_string(),
            },
            // export artifacts
            // save cache
            Command::StoreCache {
                key_files: vec!["Cargo.lock".to_string()],
                data_paths: vec![
                    "~/.cargo".to_string(),
                    "target".to_string()
                ]
            },
        ],
    };
}

// #[derive(Debug, Serialize)]
// pub struct CciYml {
//     pub version: String,
//     pub jobs: Value,
// }
