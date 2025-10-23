use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command, arg, command, value_parser};

use crate::sorting::SortingMode;

pub fn get_cli() -> Command {
    let long = command!("long").about("Print the long format");
    let tree = command!("tree").about("Print the tree format");
    let table = command!("table").about("Print the table format");

    command!()
        .subcommands([long, tree, table])
        .arg(
            arg!([path] "Path to directories")
                .value_parser(value_parser!(PathBuf))
                .default_value("./")
                .num_args(1..)
                .global(true),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Display hidden files"),
        )
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("Respect .gitignore files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("icons")
                .short('i')
                .long("icons")
                .help("Use Nerd Font icons")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-suffix")
                .short('n')
                .long("no-suffix")
                .help("Disable filename suffixes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recurse")
                .short('r')
                .long("recurse")
                .action(ArgAction::SetTrue)
                .help("Recurse into subdirectories"),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .help("Depth to recurse to")
                .value_parser(value_parser!(usize))
                .requires("recurse")
                .default_value("0"),
        )
        .arg(
            Arg::new("sort")
                .long("sort")
                .short('s')
                .help("Set the sorting mode")
                .value_name("mode")
                .value_parser(value_parser!(SortingMode))
                .default_value("name")
                .conflicts_with_all(["size", "unsorted", "mod"]),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('R')
                .help("Reverse sorted files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("mod")
                .long("mod")
                .short('m')
                .help("Sort by modified time")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["size", "unsorted", "sort"]),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .short('S')
                .help("Sort by file size")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["unsorted", "mod", "sort"]),
        )
        .arg(
            Arg::new("unsorted")
                .long("unsorted")
                .short('u')
                .help("Sort files by directory order")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["size", "mod", "sort"]),
        )
}

pub fn get_sorting_mode(matches: &ArgMatches) -> SortingMode {
    if get_bool(matches, "mod") {
        return SortingMode::Time;
    }
    if get_bool(matches, "size") {
        return SortingMode::Size;
    }
    if get_bool(matches, "unsorted") {
        return SortingMode::None;
    }

    matches
        .get_one::<SortingMode>("sort")
        .copied()
        .unwrap_or_default()
}

fn get_bool(matches: &ArgMatches, id: &str) -> bool {
    matches.get_one::<bool>(id).copied().unwrap_or_default()
}
