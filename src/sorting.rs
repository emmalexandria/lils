use std::{fs, os::unix::fs::MetadataExt, path::Path};

use clap::ValueEnum;

use crate::files::{Directory, File};

#[derive(Clone, Copy, ValueEnum, Debug, Default)]
pub enum SortingMode {
    Time,
    #[default]
    Name,
    Size,
    None,
}

pub fn sort(files: &Vec<File>, mode: SortingMode, reverse: bool) -> Vec<File> {
    let mut output = match mode {
        SortingMode::Time => time_sort(files),
        SortingMode::Name => name_sort(files),
        SortingMode::Size => size_sort(files),
        SortingMode::None => files.clone(),
    };

    if reverse {
        output.reverse();
    }

    output
}

fn time_sort(files: &Vec<File>) -> Vec<File> {
    let mut output = files.clone();
    output.sort_by(|a, b| a.times.modified.cmp(&b.times.modified));
    output
}

fn name_sort(files: &Vec<File>) -> Vec<File> {
    let mut output = files.clone();
    output.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    output
}

fn size_sort(files: &Vec<File>) -> Vec<File> {
    let mut output = files.clone();
    output.sort_by(|a, b| get_file_size(&b.path).cmp(&get_file_size(&a.path)));
    output
}

fn get_file_size<P: AsRef<Path>>(path: &P) -> u64 {
    let path = path.as_ref();
    if path.is_dir() {
        get_dir_size(path)
    } else {
        fs::metadata(path).unwrap().size()
    }
}

fn get_dir_size(path: &Path) -> u64 {
    let mut size = 0;
    let read_dir = fs::read_dir(path).unwrap();

    for entry in read_dir.flatten() {
        if entry.file_type().unwrap().is_dir() {
            size += get_dir_size(&entry.path())
        } else {
            size += entry.metadata().unwrap().size()
        }
    }

    size
}
