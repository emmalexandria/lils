use std::os::unix::fs::PermissionsExt;

use crossterm::{style::Stylize, terminal::size};
use nix::sys::stat::Mode;

use crate::{
    config::Config,
    files::{EntryType, FileType, FsEntry},
    output::MultiStyled,
    sorting::sort,
    style::{LilsStyle, ls_style},
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

    let mut lines: Vec<String> = Vec::new();

    let children = root.children.clone().unwrap();
    let (width, _) = size().unwrap_or((160, 0));

    let files = sort(&children, config.sorting.mode, config.sorting.reverse);

    for f in &files {
        let mut output: MultiStyled<String> = MultiStyled::new();
        output.append(get_permission_string(f, &style));

        lines.push(output.output());
    }

    lines.join("\n")
}

fn get_permission_string(entry: &FsEntry, style: &LilsStyle) -> MultiStyled<String> {
    let ft = match entry.e_type {
        EntryType::Directory => 'd',
        EntryType::File(ft) => match ft {
            FileType::Executable => '*',
            _ => '-',
        },
        EntryType::BlockDevice => 'b',
        EntryType::CharDevice => 'c',
        EntryType::Socket => 's',
        EntryType::Symlink => 'l',
    };

    let perms_mode: u16 = entry.perms.mode() as u16;
    let mode = nix::sys::stat::Mode::from_bits_truncate(perms_mode);

    let o_read = mode.contains(Mode::S_IRUSR);
    let o_write = mode.contains(Mode::S_IWUSR);
    let o_exec = mode.contains(Mode::S_IXUSR);

    let owner: String = get_permission_chars(o_read, o_write, o_exec)
        .iter()
        .collect();

    let g_read = mode.contains(Mode::S_IRGRP);
    let g_write = mode.contains(Mode::S_IWGRP);
    let g_exec = mode.contains(Mode::S_IXGRP);

    let group: String = get_permission_chars(g_read, g_write, g_exec)
        .iter()
        .collect();

    let o_read = mode.contains(Mode::S_IROTH);
    let o_write = mode.contains(Mode::S_IWOTH);
    let o_exec = mode.contains(Mode::S_IXOTH);

    let other: String = get_permission_chars(o_read, o_write, o_exec)
        .iter()
        .collect();

    let mut output: MultiStyled<String> = MultiStyled::new();
    let divider = "•".to_string().stylize();
    output.push(style.permissions.f_type.apply(ft.to_string()));
    output.push("┃".to_string().stylize());
    output.push(style.permissions.owner.apply(owner));
    output.push(divider.clone());
    output.push(style.permissions.group.apply(group));
    output.push(divider.clone());
    output.push(style.permissions.other.apply(other));

    output
}

fn get_permission_chars(read: bool, write: bool, exec: bool) -> [char; 3] {
    let read = if read { 'r' } else { '-' };
    let write = if write { 'w' } else { '-' };
    let exec = if exec { 'x' } else { '-' };

    [read, write, exec]
}
