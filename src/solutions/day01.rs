use itertools::Itertools;

pub fn parse_input(input_data: &str) -> Vec<Vec<u32>> {
    input_data
        .trim_end()
        .split("\n\n")
        .map(|elf| elf.lines().map(|v| v.parse().unwrap()).collect())
        .collect()
}

pub fn task1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|elv| elv.iter().cloned().sum())
        .max()
        .unwrap_or_default()
}

pub fn task2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|elv| elv.iter().cloned().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

crate::aoc_tests! {
    task1: {
        simple => 24000,
        complex => 74198,
    },
    task2: {
        simple => 45000,
        complex => 209914,
    }
}
