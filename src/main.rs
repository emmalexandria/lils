use std::{io, path::PathBuf};

use clap::ArgMatches;

use crate::{
    cli::get_cli,
    config::Config,
    files::FsEntry,
    output::{long, short},
};

mod cli;
mod config;
mod files;
mod output;
mod sorting;
mod style;
mod util;

fn main() {
    let cli = get_cli();
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("config", _)) => {
            Config::write_default();
            return;
        }
        _ => {}
    }

    let config = Config::read().unwrap().override_with_args(&matches);
    let res = display(matches, config);
}

fn display(matches: ArgMatches, config: Config) -> io::Result<()> {
    let paths: Vec<PathBuf> = matches
        .get_many::<PathBuf>("path")
        .unwrap()
        .cloned()
        .collect();

    let entries: Vec<FsEntry> = paths
        .iter()
        .flat_map(|p| FsEntry::from_path(p, &config))
        .collect();

    match matches.subcommand() {
        Some(("tree", matches)) => {
            println!("tree")
        }
        Some(("long", matches)) => long(&entries, &config),
        _ => short(&entries, &config),
    }

    Ok(())
}
