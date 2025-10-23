use crossterm::{
    style::{StyledContent, Stylize},
    terminal::size,
};

use crate::{
    config::Config,
    files::{EntryType, FsEntry},
    output::{MultiStyled, entry::display_name},
    sorting::sort,
    style::ls_style,
};

pub fn short(roots: &Vec<FsEntry>, config: &Config) {
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

fn display_single(entry: &FsEntry, config: &Config, len: usize, index: usize) {
    let output = short_display(entry, config);
    if len > 1 {
        println!("{}:", entry.name.clone().stylize().underlined().bold());
    }
    println!("{output}");
    if index < len - 1 {
        println!()
    }
}

fn short_display(root: &FsEntry, config: &Config) -> String {
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
    let files = sort(&children, config.sorting.mode, config.sorting.reverse);

    for f in &files {
        let mut output: MultiStyled<String> =
            display_name(f, &style, config.display.suffix, config.display.icons);
        pad_right(&mut output, longest_len);

        if line_pos >= files_per {
            line_pos = 0;
            lines.push(curr);
            curr = String::new();
        }

        if line_pos < files_per {
            let between = if config.display.icons { "  " } else { " " };
            output.push(between.to_string().stylize())
        }

        curr.push_str(&output.output());
        line_pos += 1;
    }

    if !curr.is_empty() {
        lines.push(curr)
    }

    lines.join("\n")
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
