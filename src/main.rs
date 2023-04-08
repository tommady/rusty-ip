mod config;
mod runner;

use std::{env, path::Path};

use log::info;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    env_logger::init();

    match parse_command() {
        Command::Help => print_help(),
        Command::ConfigPath(path) => run(&path),
    }
}

fn run(cfg_path: &str) {
    let cfg = config::read_config(Path::new(cfg_path)).expect("read config failed");
    let mut runner = runner::Runner::new(&cfg);

    ctrlc::set_handler(move || {
        info!("received exit signal");
        runner.close();
    })
    .expect("setting Ctrl-C handler failed");
}

fn print_help() {
    println!(
        "Usage rusty-ip [OPTION]... [FILE]...
Just run the program without any options will using default config settings,
Default config path is a file named `config.yaml` under the current folder.

Mandatory arguments to long options.
--config    specific the config file path"
    )
}

#[derive(Debug, PartialEq)]
enum Command {
    ConfigPath(String),
    Help,
}

fn parse_command() -> Command {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0..=1 => Command::ConfigPath(crate::config::DEFAULT_CONFIG_PATH.to_string()),
        _ => match args[1].as_ref() {
            "--config" => Command::ConfigPath(args[2].clone()),
            _ => Command::Help,
        },
    }
}
