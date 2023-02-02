use advent_of_code::helpers::parse_regex::parse_lines;
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    // read line
    // if (first[0] <= second[0] and first[1] >= second[1]) or (second[0] <= first[0] and second[1] >= first[1])
    // add 1 to count
    let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();

    let input: Vec<(usize, usize, usize, usize)> = parse_lines(re, input.trim()).collect();

    Some(
        input
            .iter()
            .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();

    let input: Vec<(usize, usize, usize, usize)> = parse_lines(re, input.trim()).collect();

    Some(
        input
            .iter()
            .filter(|(a1, a2, b1, b2)| a2 >= b1 && b2 >= a1)
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
