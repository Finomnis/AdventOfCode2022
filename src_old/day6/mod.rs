use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn solve_part1(line: &str) -> usize {
    line.trim_end()
        .as_bytes()
        .windows(4)
        .take_while(|window| {
            window.into_iter().cloned().collect::<HashSet<_>>().len() != window.len()
        })
        .count()
        + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(line: &str) -> usize {
    line.trim_end()
        .as_bytes()
        .windows(14)
        .take_while(|window| {
            window.into_iter().cloned().collect::<HashSet<_>>().len() != window.len()
        })
        .count()
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = crate::utils::day_input!(day6);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 1093);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 3534);
    }
}
