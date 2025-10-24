use std::collections::HashMap;

use crossterm::style::{ContentStyle, StyledContent, Stylize};

use crate::files::{EntryType, FileType, FsEntry};

#[derive(Default, Debug, Clone)]
pub struct LilsStyle {
    pub directory: ContentStyle,
    pub symlink: ContentStyle,
    pub socket: ContentStyle,
    pub files: HashMap<FileType, ContentStyle>,
}

impl LilsStyle {
    pub const fn directory(mut self, style: ContentStyle) -> Self {
        self.directory = style;
        self
    }

    pub const fn symlink(mut self, style: ContentStyle) -> Self {
        self.symlink = style;
        self
    }

    pub const fn socket(mut self, style: ContentStyle) -> Self {
        self.socket = style;
        self
    }

    pub fn set_ft(mut self, f_type: FileType, style: ContentStyle) -> Self {
        self.files.insert(f_type, style);
        self
    }

    pub fn apply(&self, file: &FsEntry) -> StyledContent<String> {
        let name = file.name.clone();
        match file.e_type {
            EntryType::Directory => self.directory.apply(name),
            EntryType::File(f_type) => {
                let style = self.files.get(&f_type).cloned().unwrap_or_default();
                style.apply(name)
            }
            EntryType::Symlink => self.symlink.apply(name),
            EntryType::BlockDevice => name.stylize(),
            EntryType::CharDevice => name.stylize(),
            EntryType::Socket => self.socket.apply(name),
        }
    }
}

pub fn ls_style() -> LilsStyle {
    LilsStyle::default()
        .directory(ContentStyle::default().blue().bold())
        .socket(ContentStyle::default().green())
        .set_ft(FileType::Executable, ContentStyle::default().red().bold())
}
