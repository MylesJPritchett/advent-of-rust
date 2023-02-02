use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.trim().lines().collect();
    Some(
        input
            .iter()
            .map(|line| {
                let both_sacks: Vec<char> = line.chars().collect();
                let n_items = both_sacks.len();
                let sack_1: HashSet<&char> = both_sacks[0..(n_items / 2)].iter().collect();
                let sack_2: HashSet<&char> = both_sacks[(n_items / 2)..n_items].iter().collect();
                let common_item = **sack_1.intersection(&sack_2).next().unwrap();
                priority(common_item)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.trim().lines().collect();
    Some(
        input
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .tuples()
            .map(|(sack_1, sack_2, sack_3)| {
                let sack_1_2: HashSet<char> = sack_1.intersection(&sack_2).copied().collect();
                let item = sack_1_2.intersection(&sack_3).next().unwrap();
                priority(*item)
            })
            .sum(),
    )
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        item => unreachable!("Unexpected item: {}", item),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
