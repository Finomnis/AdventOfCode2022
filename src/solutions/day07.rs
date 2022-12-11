use std::{collections::HashMap, slice::Iter};

mod parser {
    use super::{DirEntry, ShellCommand};

    pub use crate::helpers::nom::*;

    fn cd_command(input: &str) -> VResult<ShellCommand> {
        map(preceded(tag("$ cd "), not_line_ending), |val: &str| {
            ShellCommand::Cd(val.to_string())
        })(input)
    }

    fn ls_command(input: &str) -> VResult<ShellCommand> {
        map(
            preceded(tag("$ ls"), many1(preceded(line_ending, ls_entry))),
            ShellCommand::Ls,
        )(input)
    }

    fn ls_entry(input: &str) -> VResult<DirEntry> {
        alt((
            map(preceded(tag("dir "), not_line_ending), |val: &str| {
                DirEntry::Dir(val.to_string())
            }),
            map(
                separated_pair(u64, char(' '), not_line_ending),
                |(size, name): (u64, &str)| DirEntry::File(name.to_string(), size),
            ),
        ))(input)
    }

    fn shell_command(input: &str) -> VResult<ShellCommand> {
        alt((cd_command, ls_command))(input)
    }

    pub fn shell_commands(input: &str) -> VResult<Vec<ShellCommand>> {
        separated_list1(line_ending, shell_command)(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirEntry {
    Dir(String),
    File(String, u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellCommand {
    Ls(Vec<DirEntry>),
    Cd(String),
}

type DirectoryEntries = HashMap<String, Box<NodeContent>>;

#[derive(Debug)]
enum NodeContent {
    File { size: u64 },
    Directory { entries: Option<DirectoryEntries> },
}

fn parse_into_tree(shell_commands: &[ShellCommand]) -> DirectoryEntries {
    let mut shell_commands = shell_commands.iter();

    assert_eq!(
        shell_commands.next().unwrap(),
        &ShellCommand::Cd("/".to_string())
    );

    let mut root = None;

    fn visit_directory(dir: &mut Option<DirectoryEntries>, commands: &mut Iter<ShellCommand>) {
        while let Some(command) = commands.next() {
            match command {
                ShellCommand::Ls(entries) => {
                    assert!(dir.is_none());
                    *dir = Some(
                        entries
                            .iter()
                            .map(|entry| match entry {
                                DirEntry::Dir(name) => (
                                    name.clone(),
                                    Box::new(NodeContent::Directory {
                                        entries: Default::default(),
                                    }),
                                ),
                                DirEntry::File(name, size) => {
                                    (name.clone(), Box::new(NodeContent::File { size: *size }))
                                }
                            })
                            .collect(),
                    )
                }
                ShellCommand::Cd(name) if name == "/" => panic!("Unexpected 'cd /'!"),
                ShellCommand::Cd(name) if name == ".." => break,
                ShellCommand::Cd(name) => {
                    if let Some(dir_content) = dir {
                        if let NodeContent::Directory { entries: child_dir } = dir_content
                            .get_mut(name)
                            .expect("Directory does not exist!")
                            .as_mut()
                        {
                            visit_directory(child_dir, commands)
                        }
                    } else {
                        panic!("Current directory not searched yet!");
                    }
                }
            }
        }
    }

    visit_directory(&mut root, &mut shell_commands);

    root.unwrap()
}

pub fn parse_input(input_data: &str) -> Vec<ShellCommand> {
    let input_data = input_data.trim_end();
    parser::finalize(input_data)(parser::shell_commands(input_data))
}

pub fn task1(shell_commands: &[ShellCommand]) -> u64 {
    let root = parse_into_tree(shell_commands);

    //println!("{:#?}", root);

    const MAX_SIZE: u64 = 100000;

    fn get_size_and_score(_name: &str, dir: &DirectoryEntries) -> (u64, u64) {
        //println!(" -> {_name}");
        let (size, score) = dir
            .iter()
            .map(|(name, entry)| match entry.as_ref() {
                NodeContent::File { size } => (*size, 0),
                NodeContent::Directory { entries } => entries
                    .as_ref()
                    .map(|d| get_size_and_score(name, d))
                    .unwrap(),
            })
            .reduce(|(size, score), (size2, score2)| (size + size2, score + score2))
            .unwrap_or((0, 0));

        if size <= MAX_SIZE {
            (size, score + size)
        } else {
            (size, score)
        }
        //println!(" <- {:?}", result);
    }

    let (_size, score) = get_size_and_score("/", &root);

    score
}

pub fn task2(shell_commands: &[ShellCommand]) -> u64 {
    let root = parse_into_tree(shell_commands);

    //println!("{:#?}", root);

    fn get_sizes(_name: &str, dir: &DirectoryEntries) -> (u64, Vec<u64>) {
        //println!(" -> {_name}");
        let (size, mut sizes) = dir
            .iter()
            .map(|(name, entry)| match entry.as_ref() {
                NodeContent::File { size } => (*size, vec![]),
                NodeContent::Directory { entries } => {
                    entries.as_ref().map(|d| get_sizes(name, d)).unwrap()
                }
            })
            .reduce(|(size, sizes), (size2, sizes2)| {
                (
                    size + size2,
                    sizes.into_iter().chain(sizes2.into_iter()).collect(),
                )
            })
            .unwrap_or((0, vec![]));

        sizes.push(size);

        //println!(" <- {:?}", (size, sizes));
        (size, sizes)
    }

    let (size, sizes) = get_sizes("/", &root);

    const TOTAL_SPACE: u64 = 70000000;
    const NEEDED_SPACE: u64 = 30000000;
    let required_space = size - (TOTAL_SPACE - NEEDED_SPACE);

    sizes
        .into_iter()
        .filter(|&val| val >= required_space)
        .min()
        .unwrap()
}

crate::aoc_tests! {
    task1: {
        simple => 95437,
        complex => 1915606,
    },
    task2: {
        simple => 24933642,
        complex => 5025657,
    }
}
