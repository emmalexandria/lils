use std::{env, fs, io, path::PathBuf};

use clap::ArgMatches;
use serde::{Deserialize, Serialize};

use crate::{
    cli::{get_bool, get_sorting_mode},
    sorting::SortingMode,
};

const CONFIG_PATH: &str = ".config/lils.toml";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SortingConfig {
    pub mode: SortingMode,
    pub reverse: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Display {
    pub icons: bool,
    pub suffix: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(skip)]
    pub recurse: bool,
    pub depth: usize,
    pub git: bool,
    pub hidden: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sorting: SortingConfig,
    pub display: Display,
    pub filter: Filter,
}

impl Config {
    fn get_path() -> PathBuf {
        let mut path = env::home_dir().unwrap();
        path.push(CONFIG_PATH);
        path
    }

    pub fn read() -> io::Result<Self> {
        let path = Self::get_path();

        if fs::exists(&path)? {
            let config_str = fs::read_to_string(path)?;

            let de = toml::Deserializer::parse(&config_str).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "toml configuration could not be parsed",
                )
            })?;

            let ret = Self::deserialize(de).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "could not deserialize toml")
            })?;

            Ok(ret)
        } else {
            Ok(Self::default())
        }
    }

    pub fn write_default() -> io::Result<PathBuf> {
        let path = Self::get_path();

        let default = Self::default();
        let output = toml::to_string_pretty(&default).unwrap();

        fs::write(&path, output)?;
        Ok(path)
    }

    pub fn override_with_args(mut self, matches: &ArgMatches) -> Self {
        if let Some(sorting_mode) = get_sorting_mode(matches) {
            self.sorting.mode = sorting_mode;
        }
        if let Some(recurse) = get_bool(matches, "recurse") {
            self.filter.recurse = recurse;
        }
        if let Some(reverse) = get_bool(matches, "reverse") {
            self.sorting.reverse = reverse;
        }
        if let Some(show_hidden) = get_bool(matches, "all") {
            self.filter.hidden = show_hidden;
        }
        if let Some(git) = get_bool(matches, "git") {
            self.filter.git = git;
        }
        if let Some(icons) = get_bool(matches, "icons") {
            self.display.icons = icons;
        }
        if let Some(no_suffix) = get_bool(matches, "no-suffix") {
            self.display.suffix = !no_suffix;
        }
        if let Some(depth) = matches.get_one::<usize>("depth") {
            let f_depth = if *depth == 0 { usize::MAX } else { *depth };
            self.filter.depth = f_depth;
        }

        self
    }
}
