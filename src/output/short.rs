use crossterm::{
    style::{StyledContent, Stylize},
    terminal::size,
};

use crate::{
    Config,
    files::{EntryType, FsEntry},
    output::MultiStyled,
    sorting::sort,
    style::ls_style,
};

pub fn short_display(root: &FsEntry, config: &Config) -> String {
    let style = ls_style();
    if root.children.is_none() {
        return root.name.clone();
    }

    let children = root.children.clone().unwrap();

    let (width, _) = size().unwrap_or((160, 0));
    let mut lines: Vec<String> = Vec::new();
    let mut longest_len = 1;
    let longest_file = children.iter().max_by_key(|f| f.name.len());

    if let Some(longest) = longest_file {
        longest_len = longest.name.len();
    }

    let files_per = width as usize / longest_len;

    let mut curr = String::new();
    let mut line_pos = 0;
    let files = sort(&children, config.sorting_mode, config.reverse_sort);

    for f in &files {
        let mut output: MultiStyled<String> = style.apply(f).into();
        display_file(f, &mut output);
        pad_right(&mut output, longest_len);

        if line_pos >= files_per {
            line_pos = 0;
            lines.push(curr);
            curr = String::new();
        }

        if line_pos < files_per {
            output.push(" ".to_string().stylize())
        }

        curr.push_str(&output.output());
        line_pos += 1;
    }

    if !curr.is_empty() {
        lines.push(curr)
    }

    lines.join("\n")
}

fn display_file(file: &FsEntry, content: &mut MultiStyled<String>) {
    let suffix = get_suffix(file);
    if let Some(s) = suffix {
        content.push(s.to_string().stylize())
    }
}

fn pad_right(input: &mut MultiStyled<String>, length: usize) {
    let c_length = input.len();
    let mut pad = String::from("");
    if length > c_length {
        let remaining_len = length - c_length;
        pad = (0..remaining_len).map(|_| " ").collect::<Vec<_>>().concat();
    }

    input.push(pad.stylize())
}

fn get_suffix(file: &FsEntry) -> Option<char> {
    match file.e_type {
        EntryType::Directory => Some('/'),
        EntryType::Socket => Some('='),
        _ => None,
    }
}
