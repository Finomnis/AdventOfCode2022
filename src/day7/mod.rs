use std::{collections::HashMap, slice::Iter};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
};

use crate::utils::nom::extract_nom_value;

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

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<ShellCommand> {
    let cd_command = map(preceded(tag("$ cd "), not_line_ending), |val: &str| {
        ShellCommand::Cd(val.to_string())
    });
    let ls_entry = alt((
        map(preceded(tag("dir "), not_line_ending), |val: &str| {
            DirEntry::Dir(val.to_string())
        }),
        map(
            separated_pair(u64, char(' '), not_line_ending),
            |(size, name): (u64, &str)| DirEntry::File(name.to_string(), size),
        ),
    ));
    let ls_command = map(
        preceded(tag("$ ls"), many1(preceded(line_ending, ls_entry))),
        |val| ShellCommand::Ls(val),
    );
    let shell_command = alt((cd_command, ls_command));

    extract_nom_value(input)(separated_list1(line_ending, shell_command)(input.trim()))
}

type DirectoryEntries = HashMap<String, Box<NodeContent>>;

#[derive(Debug)]
enum NodeContent {
    File { size: u64 },
    Directory { entries: Option<DirectoryEntries> },
}

fn parse_into_tree(shell_commands: &[ShellCommand]) -> DirectoryEntries {
    let mut shell_commands = shell_commands.into_iter();

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

#[aoc(day7, part1)]
pub fn solve_part1(shell_commands: &[ShellCommand]) -> u64 {
    let root = parse_into_tree(shell_commands);

    //println!("{:#?}", root);

    const MAX_SIZE: u64 = 100000;

    fn get_size_and_score(_name: &str, dir: &DirectoryEntries) -> (u64, u64) {
        //println!(" -> {_name}");
        let (size, score) = dir
            .into_iter()
            .map(|(name, entry)| match entry.as_ref() {
                NodeContent::File { size } => (*size, 0),
                NodeContent::Directory { entries } => entries
                    .as_ref()
                    .map(|d| get_size_and_score(name, d))
                    .unwrap(),
            })
            .reduce(|(size, score), (size2, score2)| (size + size2, score + score2))
            .unwrap_or((0, 0));

        let result = if size <= MAX_SIZE {
            (size, score + size)
        } else {
            (size, score)
        };
        //println!(" <- {:?}", result);
        result
    }

    let (_size, score) = get_size_and_score("/", &root);

    score
}

#[aoc(day7, part2)]
pub fn solve_part2(shell_commands: &[ShellCommand]) -> u64 {
    let root = parse_into_tree(shell_commands);

    //println!("{:#?}", root);

    fn get_sizes(_name: &str, dir: &DirectoryEntries) -> (u64, Vec<u64>) {
        //println!(" -> {_name}");
        let (size, mut sizes) = dir
            .into_iter()
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
        let result = (size, sizes);
        //println!(" <- {:?}", result);
        result
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day7);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 95437);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 1915606);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 24933642);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 5025657);
    }
}
