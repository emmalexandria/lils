use std::{env, fs, io};

use clap::ArgMatches;
use serde::{Deserialize, Serialize};

use crate::sorting::SortingMode;

const CONFIG_PATH: &'static str = ".config/lils.toml";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sorting_mode: SortingMode,
    pub reverse_sort: bool,
    pub show_hidden: bool,
    pub icons: bool,
    pub no_suffix: bool,
    pub git: bool,
    pub recurse: bool,
    pub depth: usize,
}

impl Config {
    pub fn read() -> io::Result<Self> {
        let mut path = env::home_dir().unwrap();
        path.push(CONFIG_PATH);
        let config_str = fs::read_to_string(path)?;

        let de = toml::Deserializer::parse(&config_str).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "toml configuration could not be parsed",
            )
        })?;

        let ret = Self::deserialize(de).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidInput, "could not deserialize toml")
        })?;

        Ok(ret)
    }

    pub fn override_with_args(mut self, matches: &ArgMatches) -> Self {
        self
    }
}
