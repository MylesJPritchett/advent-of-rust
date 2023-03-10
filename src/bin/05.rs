use itertools::Itertools;

type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Input = (Stack, Vec<Move>);

fn parse(input: &str) -> Input {
    let (stack_str, moves_str) = input
        .split_once("\n\n")
        .expect("Spit stack and moves error");

    let mut stack_iter = stack_str.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().expect("Error creating stack").len() / 4 + 1];

    stack_iter.for_each(|l| {
        l.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stack[i / 4].push(c);
            }
        });
    });

    let moves = moves_str
        .lines()
        .filter_map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();

    (stack, moves)
}

fn move_stacks(stack: &mut Stack, moves: &[Move], version: u16) {
    moves.iter().for_each(|(amount, from, to)| {
        let from = &mut stack[*from - 1];
        let crates = from.split_off(from.len() - amount);
        if version == 9000 {
            stack[*to - 1].extend(crates.iter().rev());
        } else {
            stack[*to - 1].extend(crates.iter());
        }
    });
}

fn get_top_row(stack: &Stack) -> String {
    stack.iter().filter_map(|m| m.last()).join("")
}
pub fn part_one(input: &str) -> Option<String> {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 9000);
    Some(get_top_row(&stack))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stack, moves) = parse(input);
    move_stacks(&mut stack, &moves, 9001);
    Some(get_top_row(&stack))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("TLFGBZHCN".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("QRQFHFWCL".to_string()));
    }
}
