use askama::Template;
use clap::{Parser, Subcommand};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

use crate::data::*;

mod data;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Select which CI system to target.
    #[arg(short, long)]
    target: Target,
    // /// Name of the person to greet
    // #[clap(short, long, value_parser)]
    // name: String,

    // /// Number of times to greet
    // #[clap(short, long, value_parser, default_value_t = 1)]
    // count: u8,
    // #[clap(subcommand)]
    // command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Pipeline {
        // /// lists test values
        // #[clap(short, long, action)]
        // list: bool,
    },
}

#[derive(Template)]
#[template(path = "circleci/default.yml")]
struct CircleCI {
    job: Job,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // let items = vec!["Option 1", "Option 2"];
    // let chosen = Select::new()
    //     .with_prompt("Which option do you prefer?")
    //     .items(&items)
    //     .interact()?;

    // for _ in 0..args.count {
    //     println!("Hello {}. {}!", args.name, items[chosen]);
    // }

    // match &args.command {
    //     Some(Commands::Pipeline {}) => {
    //         println!("'myapp test' was used, list is: {:?}", list)
    //     }
    //     None => todo!(),
    // }
    println!("Initialising pipeline");

    // let mut handlebars = Handlebars::new();
    // handlebars
    //     .register_template_file("template", "./templates/circleci/default.hbs")
    //     .unwrap();

    // discover components

    let mut components: Vec<Ecosystem> = vec![];

    let path = Path::new("Cargo.toml");
    if path.exists() {
        components.push(Ecosystem::Cargo);
    }

    // for each component:
    //   add config overrides

    // render pipeline config
    // let target = Target::CircleCI;

    match args.target {
        Target::CircleCI => {
            let config_dir = Path::new(".circleci");
            create_dir_all(config_dir).expect("create config_dir");
            let config_file = Path::new(".circleci/config.yml");

            // let input = File::open(config_file)?;
            // let object: Value = serde_yaml::from_reader(input)?;
            // println!("{object:#?}");

            let mut fd = File::create(config_file)?;
            // let mut fmt = adapter::FmtIoWriter::new(fd);
            for component in components {
                let cc = CircleCI { job: CC_TEST.clone() };
                write!(fd, "{cc}")?;
            }
            // serde_yaml::to_writer(
            //     fd,
            //     &CciYml {
            //         version: "2.1".to_string(),
            //         jobs: Value::Bool(true),
            //     },
            // )?;
        }
        Target::GitHubActions => todo!(),
        Target::GitLabCI => todo!(),
    }

    Ok(())
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
