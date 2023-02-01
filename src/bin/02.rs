enum Move {
    Rock,
    Paper,
    Scissors
}

enum OpponentChoicePossibilities {
    A,
    B,
    C,
}

enum MyChoicePossibilities {
    X,
    Y,
    Z,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    let mut opponent_choice: OpponentChoicePossibilities = input.split(' ').into_iter()
    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    todo!()
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
        assert_eq!(part_two(&input), None);
    }
}
