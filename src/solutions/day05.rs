mod parser {
    use super::MoveCommand;

    pub use crate::helpers::nom::*;

    fn container(input: &str) -> VResult<char> {
        delimited(char('['), anychar, char(']'))(input)
    }

    fn space(input: &str) -> VResult<&str> {
        tag("   ")(input)
    }

    fn maybe_container(input: &str) -> VResult<Option<char>> {
        alt((map(container, |c| Some(c)), map(space, |_| None)))(input)
    }

    fn container_line(input: &str) -> VResult<Vec<Option<char>>> {
        separated_list1(char(' '), maybe_container)(input)
    }

    fn move_line(input: &str) -> VResult<MoveCommand> {
        map(
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
        )(input)
    }

    pub fn raw_instructions(
        input: &str,
    ) -> VResult<(Vec<Vec<Option<char>>>, usize, Vec<MoveCommand>)> {
        tuple((
            separated_list1(line_ending, container_line),
            delimited(
                line_ending,
                many1_count(preceded(space1, u16)),
                pair(space0, count(line_ending, 2)),
            ),
            separated_list1(line_ending, move_line),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct MoveCommand {
    count: usize,
    origin: usize,
    target: usize,
}

pub fn parse_input(input_data: &str) -> (Vec<Vec<char>>, Vec<MoveCommand>) {
    let input_data = input_data.trim_end();

    let (rows, column_count, move_commands) =
        parser::finalize(input_data)(parser::raw_instructions(input_data));

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

pub fn task1((containers, move_commands): &(Vec<Vec<char>>, Vec<MoveCommand>)) -> String {
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

pub fn task2((containers, move_commands): &(Vec<Vec<char>>, Vec<MoveCommand>)) -> String {
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

crate::aoc_tests! {
    task1: {
        simple => "CMZ",
        complex => "TLNGFGMFN",
    },
    task2: {
        simple => "MCD",
        complex => "FGLQJCMBD",
    }
}
