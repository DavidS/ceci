use clap::ValueEnum;

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

const CC_TEST: Job = Job {
    ecosystem: Ecosystem::Cargo,
    steps: vec![Command {
        title: "install protoc".to_string(),
        command: "sudo apt install -y protobuf-compiler".to_string(),
    }],
};
