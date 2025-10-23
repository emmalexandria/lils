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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub e_type: EntryType,
    pub times: Times,
}

impl File {
    pub fn new<P: AsRef<Path>>(path: P, e_type: EntryType, times: Times) -> Self {
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
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let ext = path.extension().unwrap_or(OsStr::new("")).to_string_lossy();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let metadata = fs::metadata(path)?;
        let e_type = EntryType::from_metadata(&metadata, &ext);
        let times = Times {
            access: metadata.accessed().unwrap(),
            created: metadata.created().unwrap(),
            modified: metadata.modified().unwrap(),
        };

        Ok(Self::new(path, e_type, times))
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub files: Vec<File>,
    pub children: Vec<Rc<Directory>>,
    pub name: String,
}

impl Directory {
    pub fn from_path<P: AsRef<Path>>(path: P, recurse: bool, max_depth: usize) -> io::Result<Self> {
        Self::build_from_path(path, recurse, max_depth, 0)
    }

    fn build_from_path<P: AsRef<Path>>(
        path: P,
        recurse: bool,
        max_depth: usize,
        depth: usize,
    ) -> io::Result<Self> {
        let path = path.as_ref();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut files: Vec<File> = Vec::new();
        let mut children: Vec<Rc<Directory>> = Vec::new();
        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "directory cannot be built from path to file",
            ));
        }

        let rd = fs::read_dir(path)?;

        for entry in rd.flatten() {
            let ft = entry.file_type()?;
            if ft.is_file() || ft.is_symlink() {
                files.push(File::from_path(entry.path())?);
            } else if ft.is_dir() && recurse && depth < max_depth {
                children.push(Rc::new(Directory::build_from_path(
                    entry.path(),
                    recurse,
                    max_depth,
                    depth + 1,
                )?));
            }
        }

        Ok(Self {
            name,
            files,
            children,
        })
    }

    pub fn from_files<I: Iterator<Item = File>, P: AsRef<Path>>(
        files: I,
        path: P,
    ) -> io::Result<Self> {
        let path = path.as_ref();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        Ok(Self {
            files: files.collect(),
            name: name.to_string(),
            children: Vec::new(),
        })
    }
}
