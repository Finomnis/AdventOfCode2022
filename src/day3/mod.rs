use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
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

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();

    lines
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day3);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 157);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 8053);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 70);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 2425);
    }
}
