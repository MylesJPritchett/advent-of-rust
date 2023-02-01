use advent_of_code::helpers::parse_regex;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    Some(parsed_input.iter().map(compute_score_a).sum::<u64>())
}

fn compute_score_a(round: &(char, char)) -> u64 {
    let winner_score = match round {
        &('A', 'Y') | &('B', 'Z') | &('C', 'X') => 6,
        &('A', 'X') | &('B', 'Y') | &('C', 'Z') => 3,
        _ => 0,
    };

    let choice_score = match round.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        c => unreachable!("Unexpected play: {c}"),
    };

    winner_score + choice_score
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    Some(parsed_input.iter().map(compute_score_b).sum::<u64>())
}

fn compute_score_b(round: &(char, char)) -> u64 {
    let choice_score = match round {
        &('A', 'Y') | &('B', 'X') | &('C', 'Z') => 1, //rock
        &('A', 'Z') | &('B', 'Y') | &('C', 'X') => 2, //paper
        &('A', 'X') | &('B', 'Z') | &('C', 'Y') => 3, //scissors
        _ => 0,
    };

    let winner_score = match round.1 {
        'X' => 0, //lose
        'Y' => 3, //draw
        'Z' => 6, //win
        c => unreachable!("Unexpected play: {c}"),
    };

    winner_score + choice_score
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    let re = Regex::new("(A|B|C) (X|Y|Z)").unwrap();
    parse_regex::parse_lines(re, input).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
