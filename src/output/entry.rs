use crossterm::style::Stylize;

use crate::{
    files::{EntryType, FileType, FsEntry},
    output::MultiStyled,
    style::LilsStyle,
};

pub fn display_name(entry: &FsEntry, style: &LilsStyle) -> MultiStyled<String> {
    let applied = style.apply(entry);
    let mut multi: MultiStyled<String> = applied.into();
    if let Some(suffix) = get_suffix(entry) {
        multi.push(suffix.to_string().stylize());
    }

    multi
}

pub fn get_suffix(entry: &FsEntry) -> Option<char> {
    match entry.e_type {
        EntryType::Directory => Some('/'),
        EntryType::Socket => Some('='),
        EntryType::File(e_type) => match e_type {
            FileType::Executable => Some('*'),
            _ => None,
        },
        _ => None,
    }
}
