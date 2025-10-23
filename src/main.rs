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
mod files;
mod output;
mod sorting;
mod style;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub sorting_mode: SortingMode,
    pub reverse_sort: bool,
    pub show_hidden: bool,
    pub recurse: bool,
    pub depth: usize,
}

fn main() {
    let cli = get_cli();
    let matches = cli.get_matches();

    let in_depth = matches
        .get_one::<usize>("depth")
        .copied()
        .unwrap_or_default();

    let depth = if in_depth == 0 { usize::MAX } else { in_depth };

    let config = Config {
        sorting_mode: get_sorting_mode(&matches),
        reverse_sort: matches
            .get_one::<bool>("reverse")
            .copied()
            .unwrap_or_default(),
        recurse: matches
            .get_one::<bool>("recurse")
            .copied()
            .unwrap_or_default(),
        show_hidden: matches.get_one::<bool>("all").copied().unwrap_or_default(),
        depth,
    };

    let res = print_paths(matches, config);
}

fn print_paths(matches: ArgMatches, config: Config) -> io::Result<()> {
    let paths: Vec<PathBuf> = matches
        .get_many::<PathBuf>("path")
        .unwrap()
        .cloned()
        .collect();

    let entries: Vec<FsEntry> = paths
        .iter()
        .flat_map(|p| FsEntry::from_path(p, &config))
        .collect();

    short(&entries, &config);

    Ok(())
}
