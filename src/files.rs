use std::{
    env,
    ffi::OsStr,
    fs, io,
    os::unix::fs::FileTypeExt,
    path::{Path, PathBuf},
    rc::Rc,
    time,
};

use crate::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Directory,
    File(FileType),
    Symlink,
    Socket,
}

impl EntryType {
    fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        let ext = path.extension().unwrap_or(OsStr::new("")).to_string_lossy();

        Ok(Self::from_metadata(&metadata, ext.as_ref()))
    }

    fn from_metadata(metadata: &fs::Metadata, ext: &str) -> Self {
        if metadata.file_type().is_dir() {
            return Self::Directory;
        }

        if metadata.file_type().is_file() {
            return Self::File(FileType::from_ext(ext));
        }

        if metadata.file_type().is_symlink() {
            return Self::Symlink;
        }

        if metadata.file_type().is_socket() {
            return Self::Socket;
        }

        Self::File(FileType::Text)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FileType {
    Text,
}

impl FileType {
    pub fn from_ext<S: ToString>(ext: S) -> Self {
        match ext.to_string().as_str() {
            _ => Self::Text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Times {
    pub modified: time::SystemTime,
    pub created: time::SystemTime,
    pub access: time::SystemTime,
}

pub type EntryChildren = Vec<Rc<FsEntry>>;

#[derive(Debug, Clone)]
pub struct FsEntry {
    pub name: String,
    pub path: PathBuf,
    pub e_type: EntryType,
    pub times: Times,
    pub children: Option<EntryChildren>,
}

impl FsEntry {
    pub fn new<P: AsRef<Path>>(
        path: P,
        e_type: EntryType,
        times: Times,
        children: Option<Vec<Rc<FsEntry>>>,
    ) -> Self {
        let path = path.as_ref();
        let name = path
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();

        Self {
            name,
            path: path.into(),
            e_type,
            times,
            children,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<Self> {
        let path = path.as_ref();
        let ext = path.extension().unwrap_or(OsStr::new("")).to_string_lossy();
        let metadata = fs::metadata(path)?;
        let e_type = EntryType::from_metadata(&metadata, &ext);
        let times = Times {
            access: metadata.accessed().unwrap(),
            created: metadata.created().unwrap(),
            modified: metadata.modified().unwrap(),
        };

        let mut children = None;

        if e_type == EntryType::Directory {
            children = Some(Self::get_children(path, config)?);
        }

        Ok(Self::new(path, e_type, times, children))
    }

    fn get_children<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<EntryChildren> {
        let path = path.as_ref();
        let rd = fs::read_dir(path)?;
        let mut children = Vec::new();

        for e in rd.flatten() {
            children.push(Rc::new(Self::from_path(e.path(), config)?));
        }

        Ok(children)
    }
}
