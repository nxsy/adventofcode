//! Day 07
//!
//!

use std::cmp::Reverse;

use relative_path::{Component, RelativePath as Path, RelativePathBuf as PathBuf};

use adventofcode2022::prelude::*;
use strum::EnumDiscriminants;
use strum_macros::EnumString;

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, EnumDiscriminants, Clone)]
#[strum_discriminants(derive(EnumString))]
enum Command {
    Cd { relative_dir: String },
    Ls,
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with("$ ") {
            panic!("Unexpected non-command");
        }

        let (_, cmd) = value
            .split_once(' ')
            .context("Couldn't separate out string")?;

        let (bare_cmd, args) = match cmd.split_once(' ') {
            None => (cmd, None),
            Some((bare_cmd, args)) => (bare_cmd, Some(args)),
        };
        match bare_cmd {
            "ls" => Ok(Command::Ls),
            "cd" => Ok(Command::Cd {
                relative_dir: args.unwrap().to_owned(),
            }),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
enum Entry {
    Directory {
        entries: HashMap<String, Entry>,
        size: usize,
    },
    File {
        size: usize,
    },
}
impl Entry {
    fn new_directory() -> Entry {
        Entry::Directory {
            entries: HashMap::new(),
            size: 0,
        }
    }
}

#[derive(Debug)]
struct Filesystem {
    root: Entry,
}

impl Filesystem {
    fn new() -> Filesystem {
        Filesystem {
            root: Entry::new_directory(),
        }
    }

    fn get_entries(&mut self, dir: &Path) -> &mut HashMap<String, Entry> {
        let mut curdir = &mut self.root;
        for component in dir.components() {
            if let Component::Normal(c) = component {
                curdir = match curdir {
                    Entry::Directory { entries, .. } => {
                        if let Some(curdir) = entries.get_mut(c) {
                            curdir
                        } else {
                            panic!("Unknown path");
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!();
            }
        }
        match curdir {
            Entry::Directory { entries, .. } => entries,
            Entry::File { .. } => unreachable!(),
        }
    }

    fn add_dir(&mut self, dir: &Path, dirname: &str) {
        let entries = self.get_entries(dir);
        entries.insert(dirname.to_string(), Entry::new_directory());
    }

    fn add_file(&mut self, dir: &Path, filename: &str, size: usize) {
        let mut curdir = &mut self.root;
        for component in dir.components() {
            if let Component::Normal(c) = component {
                curdir = match curdir {
                    Entry::Directory { entries, .. } => {
                        if let Some(curdir) = entries.get_mut(c) {
                            curdir
                        } else {
                            panic!("Unknown path");
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!();
            }
        }
        match curdir {
            Entry::Directory { entries, .. } => {
                entries.insert(filename.to_string(), Entry::File { size });
            }
            Entry::File { .. } => unreachable!(),
        }
    }

    fn populate_sizes(&mut self) {
        fn populate_size_recursive(entries: &mut HashMap<String, Entry>) -> usize {
            let mut size = 0_usize;
            for entry in entries {
                size += match entry.1 {
                    Entry::Directory { entries, size } => {
                        *size = populate_size_recursive(entries);
                        *size
                    }
                    Entry::File { size } => *size,
                }
            }
            size
        }

        match &mut self.root {
            Entry::Directory { entries, size } => {
                *size = populate_size_recursive(entries);
            }
            _ => unreachable!(),
        }
    }

    fn sum_of_dirs_smaller_than_n(&self, n: usize) -> usize {
        fn sum_of_dirs_smaller_than_n_recursive(
            entries: &HashMap<String, Entry>,
            n: usize,
        ) -> usize {
            let mut size = 0_usize;
            for entry in entries {
                size += match entry.1 {
                    Entry::Directory { entries, size } => {
                        sum_of_dirs_smaller_than_n_recursive(entries, n)
                            + if *size < n { *size } else { 0 }
                    }
                    Entry::File { .. } => 0,
                }
            }
            size
        }
        match &self.root {
            Entry::Directory { entries, .. } => sum_of_dirs_smaller_than_n_recursive(entries, n),
            _ => unreachable!(),
        }
    }

    fn total_size(&self) -> usize {
        match &self.root {
            Entry::Directory { entries: _, size } => *size,
            _ => unreachable!(),
        }
    }

    fn smallest_dirs(&self) -> BinaryHeap<Reverse<usize>> {
        fn smallest_dirs_recursive(
            entries: &HashMap<String, Entry>,
            heap: &mut BinaryHeap<Reverse<usize>>,
        ) {
            for entry in entries {
                if let Entry::Directory { entries, size } = entry.1 {
                    heap.push(Reverse(*size));
                    smallest_dirs_recursive(entries, heap);
                }
            }
        }

        let mut heap = BinaryHeap::new();
        match &self.root {
            Entry::Directory { entries, size } => {
                heap.push(Reverse(*size));
                smallest_dirs_recursive(entries, &mut heap);
            }
            _ => unreachable!(),
        }
        heap
    }
}

fn part1(file_data: &str) -> Result<()> {
    let mut fs = Filesystem::new();

    fs_from_string(file_data, &mut fs)?;
    fs.populate_sizes();
    let size = fs.sum_of_dirs_smaller_than_n(100000);
    dbg!(size);
    Ok(())
}

fn fs_from_string(file_data: &str, fs: &mut Filesystem) -> Result<(), anyhow::Error> {
    let mut curdir = PathBuf::from("");
    let mut lines = file_data.lines().peekable();
    loop {
        let command = Command::try_from(lines.next().unwrap())?;
        if let Command::Cd { relative_dir } = &command {
            curdir = curdir.join(relative_dir).normalize();
        }
        while matches!(lines.peek(), Some(l) if !l.starts_with("$ ")) {
            match lines.next() {
                Some(l) if !l.starts_with("$ ") => {
                    let (a, b) = l.split_once(' ').context("ls output parse fail")?;
                    if a == "dir" {
                        fs.add_dir(&curdir, b);
                    } else if let Ok(size) = a.parse::<usize>() {
                        fs.add_file(&curdir, b, size);
                    } else {
                        unimplemented!();
                    }
                }
                _ => unreachable!(),
            }
        }
        match lines.peek() {
            Some(l) if l.starts_with("$ ") => continue,
            Some(_) => unreachable!(),
            None => break,
        }
    }
    Ok(())
}

fn part2(file_data: &str) -> Result<()> {
    let mut fs = Filesystem::new();

    fs_from_string(file_data, &mut fs)?;
    fs.populate_sizes();
    let available = 70000000;
    let unused = available - fs.total_size();
    let needed = 30000000 - unused;
    let mut smallest_dirs = fs.smallest_dirs();
    while let Some(Reverse(x)) = smallest_dirs.pop() {
        if x > needed {
            println!("Delete {x}!");
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::FinalInput => include_str!("input"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
