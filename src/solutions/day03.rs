pub fn parse_input(input_data: &str) -> Vec<&str> {
    input_data.trim_end().lines().collect()
}

pub fn task1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .inspect(|(left, right)| assert!(left.len() == right.len()))
        .map(|(left, right)| {
            let duplicate = left
                .chars()
                .filter(|&ch| right.contains(ch))
                .next()
                .unwrap();
            match duplicate {
                'a'..='z' => duplicate as u32 - 'a' as u32 + 1,
                'A'..='Z' => duplicate as u32 - 'A' as u32 + 27,
                _ => panic!("Invalid item encountered!"),
            }
        })
        .sum()
}

pub fn task2(input: &[&str]) -> u32 {
    input
        .chunks(3)
        .map(|slice| <&[_; 3]>::try_from(slice).unwrap())
        .map(|&[a, b, c]| {
            let duplicate = a
                .chars()
                .filter(|&ch| b.contains(ch) && c.contains(ch))
                .next()
                .unwrap();
            match duplicate {
                'a'..='z' => duplicate as u32 - 'a' as u32 + 1,
                'A'..='Z' => duplicate as u32 - 'A' as u32 + 27,
                _ => panic!("Invalid item encountered!"),
            }
        })
        .sum()
}

crate::aoc_tests! {
    task1: {
        simple => 157,
        complex => 8053,
    },
    task2: {
        simple => 70,
        complex => 2425,
    }
}
