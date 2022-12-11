use std::ops::RangeInclusive;

mod parser {
    use std::ops::RangeInclusive;

    pub use crate::helpers::nom::*;

    pub fn line(input: &str) -> VResult<(RangeInclusive<u32>, RangeInclusive<u32>)> {
        separated_pair(
            map(separated_pair(u32, char('-'), u32), |(s, e)| s..=e),
            char(','),
            map(separated_pair(u32, char('-'), u32), |(s, e)| s..=e),
        )(input)
    }
}

pub fn parse_input(input_data: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input_data
        .trim_end()
        .lines()
        .map(parser::line)
        .map(parser::finalize(input_data))
        .collect()
}

pub fn task1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            (b.contains(a.start()) && b.contains(a.end()))
                || (a.contains(b.start()) && a.contains(b.end()))
        })
        .count()
}

pub fn task2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start())
                || b.contains(a.end())
                || a.contains(b.start())
                || a.contains(b.end())
        })
        .count()
}

crate::aoc_tests! {
    task1: {
        simple => 2,
        complex => 459,
    },
    task2: {
        simple => 4,
        complex => 779,
    }
}
