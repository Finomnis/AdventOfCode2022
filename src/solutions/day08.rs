mod parser {
    pub use crate::helpers::nom::*;

    fn _single_digit(input: &str) -> VResult<u8> {
        map(one_of("0123456789"), |ch| ch as u8 - b'0')(input)
    }
}

pub fn parse_input(input_data: &str) -> Vec<()> {
    let _input_data = input_data.trim_end();
    todo!()
}

pub fn task1(_forest: &[()]) -> u64 {
    0
}

pub fn task2(_forest: &[()]) -> u64 {
    0
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
