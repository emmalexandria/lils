use crossterm::{style::Stylize, terminal::size};

use crate::{
    config::Config,
    files::{EntryType, FileType, FsEntry},
    output::MultiStyled,
    style::ls_style,
};

pub fn long(roots: &[FsEntry], config: &Config) {
    if config.filter.recurse {
        let all = roots.iter().map(|e| e.get_all_dirs());

        all.for_each(|r| {
            r.iter().enumerate().for_each(|(i, e)| {
                display_single(e, config, r.len(), i);
            });
        });
    } else {
        roots.iter().enumerate().for_each(|(i, e)| {
            display_single(e, config, roots.len(), i);
        });
    }
}

fn display_single(entry: &FsEntry, config: &Config, len: usize, idx: usize) {
    let output = long_display(entry, config);
    if len > 1 {
        println!("{}:", entry.name.clone().stylize().underlined().bold());
    }
    println!("{output}");
    if idx < len - 1 {
        println!();
    }
}

fn long_display(root: &FsEntry, config: &Config) -> String {
    let style = ls_style();
    if root.children.is_none() {
        return root.name.clone();
    }

    let lines: Vec<String> = Vec::new();

    let children = root.children.clone().unwrap();
    let (width, _) = size().unwrap_or((160, 0));

    for f in &files {
        let mut output: MultiStyled<String>;
    }

    lines.join("\n")
}

fn get_permission_string(entry: &FsEntry) -> String {
    let ft = match entry.e_type {
        EntryType::Directory => 'd',
        EntryType::File(ft) => match ft {
            FileType::Executable => '*',
            _ => '-',
        },
        EntryType::Socket => 's',
        EntryType::Symlink => 'l',
    };
}
