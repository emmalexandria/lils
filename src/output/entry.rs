use crossterm::style::Stylize;

use crate::{
    files::{EntryType, FileType, FsEntry},
    output::MultiStyled,
    style::LilsStyle,
};

pub fn display_name(
    entry: &FsEntry,
    style: &LilsStyle,
    suffix: bool,
    icons: bool,
) -> MultiStyled<String> {
    let applied = style.apply(entry);
    let mut multi: MultiStyled<String> = applied.into();
    if suffix && let Some(suffix) = get_suffix(entry) {
        multi.push(suffix.to_string().stylize());
    }
    if icons && let Some(icon_raw) = get_icon(entry) {
        let icon = format!("{icon_raw} ");
        multi.insert(0, icon.stylize());
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

pub fn get_icon(entry: &FsEntry) -> Option<char> {
    match entry.e_type {
        EntryType::Directory => Some(''),
        EntryType::File(ft) => match ft {
            FileType::Executable => Some(''),
            FileType::Text => Some('󰈚'),
            _ => None,
        },
        _ => None,
    }
}
