<h1 align="center">lils</h1>
<p align="center"><i>lils is not ls</i></p>

`lils` is a modern Rust replacement for `ls`. As the name implies, this is not an `ls` rewrite, but a modern replacement for it. It aims to near feature parity
alongside new features, but it is *not* a drop-in replacement. This is the selling point.

## Motivation
Like many of the coreutils, `ls` has some... questionable design choices when it comes to usability. Given that `ls` is a user-facing command, 
this is not an ideal situation. Seeing this:
```
usage: ls [-@ABCFGHILOPRSTUWXabcdefghiklmnopqrstuvwxy1%,] [--color=when] [-D format] [file ...]
``` 

is not exactly friendly, and man pages are great but quite a pain when you're learning. `lils` aims to be far 
friendlier than this both in input and output. 

`lils` does aim to be usable on TTY and for scripts, but these use-cases are low priority. 

## Features
- Modern and useful help output
- Intuitive, beginner friendly command line arguments and subcommands
- More output modes
- File-type based colouring beyond directory, file, socket, and symlink
- Fully configurable output incl. themes
- More extensive use of coloured and styled output

## Usage
If you have the Rust toolchain installed, run:
```
cargo install lils
```
Pre-built binaries are provided for MacOS and Linux. Windows support is on the roadmap, but not presently implemented.

```
lils is not ls. modern directory listing CLI.

Usage: lils [OPTIONS] [path]... [COMMAND]

Commands:
  long      Print the long format
  tree      Print the tree format
  explorer  Interactive explorer mode
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [path]...  Path to directories [default: ./]

Options:
  -a, --all            Display hidden files
  -g, --git            Respect .gitignore files
  -r, --recurse        Recurse into subdirectories
  -d, --depth <depth>  Depth to recurse to [default: 0]
  -s, --sort <mode>    Set the sorting mode [default: name] [possible values: time, name, size, none]
  -R, --reverse        Reverse sorted files
  -m, --mod            Sort by modified time
  -S, --size           Sort by file size
  -u, --unsorted       Sort files by directory order
  -h, --help           Print help
  -V, --version        Print version
```


## Configuration
