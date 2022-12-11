use std::collections::HashSet;

pub fn parse_input(input_data: &str) -> &[u8] {
    input_data.trim_end().as_bytes()
}

pub fn task1(line: &[u8]) -> usize {
    line.windows(4)
        .take_while(|window| window.iter().cloned().collect::<HashSet<_>>().len() != window.len())
        .count()
        + 4
}

pub fn task2(line: &[u8]) -> usize {
    line.windows(14)
        .take_while(|window| window.iter().cloned().collect::<HashSet<_>>().len() != window.len())
        .count()
        + 14
}

crate::aoc_tests! {
    task1: {
        simple1 => 7,
        simple2 => 5,
        simple3 => 6,
        simple4 => 10,
        simple5 => 11,
        complex => 1093,
    },
    task2: {
        simple1 => 19,
        simple2 => 23,
        simple3 => 23,
        simple4 => 29,
        simple5 => 26,
        complex => 3534,
    }
}
