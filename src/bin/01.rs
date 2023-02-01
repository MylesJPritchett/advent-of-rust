pub fn part_one(input: &str) -> Option<u32> {
    let mut elf_calories: Vec<u32> = Vec::new();
    let mut calorie_total: u32 = 0;
    for c in input.lines(){
        if c.is_empty(){
            elf_calories.push(calorie_total);
            calorie_total = 0;
        } else {
            calorie_total += c.parse::<u32>().unwrap();
        }
    }
    elf_calories.sort();
    elf_calories.reverse();
    Some(elf_calories[0]) 
   
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories: Vec<u32> = Vec::new();
    let mut calorie_total: u32 = 0;
    for c in input.lines(){
        if c.is_empty(){
            elf_calories.push(calorie_total);
            calorie_total = 0;
        } else {
            calorie_total += c.parse::<u32>().unwrap();
        }
    }
    elf_calories.sort();
    elf_calories.reverse();
    Some(elf_calories[0] + elf_calories[1] + elf_calories[2]) 
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
