use std::{
    ffi::OsStr,
    fs::{self, Permissions},
    io,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    rc::Rc,
    time,
};

use ignore::WalkBuilder;

use crate::{
    config::Config,
    util::{get_group_by_gid, get_user_by_uid},
};

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

        Self::from_metadata(&metadata, path)
    }

    fn from_metadata(metadata: &fs::Metadata, path: &Path) -> io::Result<Self> {
        if metadata.file_type().is_dir() {
            return Ok(Self::Directory);
        }

        if metadata.file_type().is_file() {
            return Ok(Self::File(FileType::from_path(path)?));
        }

        if metadata.file_type().is_symlink() {
            return Ok(Self::Symlink);
        }

        if metadata.file_type().is_socket() {
            return Ok(Self::Socket);
        }

        Ok(Self::File(FileType::Text))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FileType {
    Text,
    Executable,
}

impl FileType {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let permissions = fs::metadata(path)?.permissions();

        if permissions.mode() & 0o111 != 0 {
            return Ok(Self::Executable);
        }

        Ok(match ext.as_str() {
            "txt" | "md" => Self::Text,
            _ => Self::Text,
        })
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
    pub uid: u32,
    pub u_name: Option<String>,
    pub gid: u32,
    pub g_name: Option<String>,
    pub perms: Permissions,
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
        uid: u32,
        gid: u32,
        perms: Permissions,
    ) -> Self {
        let path = path.as_ref();
        let name = path
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();

        Self {
            name,
            uid,
            u_name: get_user_by_uid(uid),
            gid,
            g_name: get_group_by_gid(gid),
            perms,
            path: path.into(),
            e_type,
            times,
            children,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<Self> {
        Self::create_from_path(path, config, 0)
    }

    fn create_from_path<P: AsRef<Path>>(
        path: P,
        config: &Config,
        depth: usize,
    ) -> io::Result<Self> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        let e_type = EntryType::from_metadata(&metadata, path)?;
        let times = Times {
            access: metadata.accessed().unwrap(),
            created: metadata.created().unwrap(),
            modified: metadata.modified().unwrap(),
        };

        let mut children = None;

        if e_type == EntryType::Directory && config.filter.recurse && depth < config.filter.depth
            || depth == 0
        {
            children = Some(Self::get_children(path, config, depth)?);
        }

        Ok(Self::new(
            path,
            e_type,
            times,
            children,
            metadata.uid(),
            metadata.gid(),
            metadata.permissions(),
        ))
    }

    fn get_children<P: AsRef<Path>>(
        path: P,
        config: &Config,
        depth: usize,
    ) -> io::Result<EntryChildren> {
        let path = path.as_ref();
        let walk = WalkBuilder::new(path)
            .hidden(config.filter.hidden)
            .ignore(false)
            .require_git(true)
            .git_ignore(config.filter.git)
            .max_depth(Some(1))
            .build();
        let mut children = Vec::new();

        for e in walk.flatten() {
            if e.file_name() != "./" {
                children.push(Rc::new(Self::create_from_path(
                    e.path(),
                    config,
                    depth + 1,
                )?));
            }
        }

        Ok(children)
    }

    pub fn get_all_dirs(&self) -> Vec<Rc<Self>> {
        let mut output = Vec::new();
        if let Some(c) = self.children.as_ref() {
            let children = c.iter().filter(|e| e.children.is_some()).cloned();
            output.extend(children);
            let child_children = c.iter().map(|c| c.get_all_dirs());
            child_children.for_each(|c| output.extend(c));
        }

        output
    }
}
