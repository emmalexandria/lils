use std::{io, path::PathBuf};

use clap::ArgMatches;
use crossterm::style::Stylize;

use crate::{
    cli::{get_cli, get_sorting_mode},
    files::FsEntry,
    output::short,
    sorting::SortingMode,
};

mod cli;
mod config;
mod files;
mod output;
mod sorting;
mod style;

#[derive(Clone, Copy, Debug)]
pub struct ConfigArgs {
    pub sorting_mode: SortingMode,
    pub reverse_sort: bool,
    pub show_hidden: bool,
    pub icons: bool,
    pub no_suffix: bool,
    pub git: bool,
    pub recurse: bool,
    pub depth: usize,
}

impl From<&ArgMatches> for ConfigArgs {
    fn from(matches: &ArgMatches) -> Self {
        let in_depth = matches
            .get_one::<usize>("depth")
            .copied()
            .unwrap_or_default();

        let depth = if in_depth == 0 { usize::MAX } else { in_depth };

        ConfigArgs {
            sorting_mode: get_sorting_mode(matches),
            reverse_sort: matches
                .get_one::<bool>("reverse")
                .copied()
                .unwrap_or_default(),
            recurse: matches
                .get_one::<bool>("recurse")
                .copied()
                .unwrap_or_default(),
            show_hidden: matches.get_one::<bool>("all").copied().unwrap_or_default(),
            git: matches.get_one::<bool>("git").copied().unwrap_or_default(),
            icons: matches
                .get_one::<bool>("icons")
                .copied()
                .unwrap_or_default(),
            no_suffix: matches
                .get_one::<bool>("no-suffix")
                .copied()
                .unwrap_or(false),
            depth,
        }
    }
}

fn main() {
    let cli = get_cli();
    let matches = cli.get_matches();

    let config = ConfigArgs::from(&matches);
    let res = display(matches, config);
}

fn display(matches: ArgMatches, config: ConfigArgs) -> io::Result<()> {
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
        Some(("long", matches)) => {}
        _ => short(&entries, &config),
    }

    Ok(())
}
