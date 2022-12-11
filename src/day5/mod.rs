use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, space0, space1, u16, u32},
    combinator::map,
    multi::{count, many1_count, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
};

use crate::utils::nom::extract_nom_value;

#[derive(Debug, Clone)]
pub struct MoveCommand {
    count: usize,
    origin: usize,
    target: usize,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<MoveCommand>) {
    let container = delimited(char('['), anychar, char(']'));
    let space = tag("   ");
    let maybe_container = alt((map(container, |c| Some(c)), map(space, |_| None)));
    let container_line = separated_list1(char(' '), maybe_container);

    let move_line = map(
        tuple((
            preceded(tag("move "), u16),
            preceded(tag(" from "), u16),
            preceded(tag(" to "), u16),
        )),
        |(count, origin, target)| MoveCommand {
            count: count.into(),
            origin: usize::from(origin) - 1,
            target: usize::from(target) - 1,
        },
    );

    let (rows, column_count, move_commands) = extract_nom_value(input)(tuple((
        separated_list1(line_ending, container_line),
        delimited(
            line_ending,
            many1_count(preceded(space1, u32)),
            pair(space0, count(line_ending, 2)),
        ),
        separated_list1(line_ending, move_line),
    ))(input));

    let mut columns = vec![vec![]; column_count];

    for row in rows.into_iter().rev() {
        for (column_id, container_space) in row.into_iter().enumerate() {
            if let Some(container) = container_space {
                columns[column_id].push(container);
            }
        }
    }

    (columns, move_commands)
}

#[aoc(day5, part1)]
pub fn solve_part1((containers, move_commands): &(Vec<Vec<char>>, Vec<MoveCommand>)) -> String {
    let mut containers = containers.clone();
    for move_command in move_commands {
        for _ in 0..move_command.count {
            let container = containers[move_command.origin].pop().unwrap();
            containers[move_command.target].push(container);
        }
    }

    containers
        .iter()
        .map(|column| column.last().unwrap())
        .collect()
}

#[aoc(day5, part2)]
pub fn solve_part2((containers, move_commands): &(Vec<Vec<char>>, Vec<MoveCommand>)) -> String {
    let mut containers = containers.clone();
    for move_command in move_commands {
        let origin = &mut containers[move_command.origin];
        let mut moved = origin.split_off(origin.len() - move_command.count);
        containers[move_command.target].append(&mut moved);
    }

    containers
        .iter()
        .map(|column| column.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day5);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), "CMZ");
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), "TLNGFGMFN");
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), "MCD");
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), "FGLQJCMBD");
    }
}
