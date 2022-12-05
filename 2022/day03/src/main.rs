fn part_one(input: &str) -> i16 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let common_type = left.chars().find(|l| right.contains(*l)).unwrap();

            let byte = common_type as u8;

            if byte >= b'a' {
                (byte - b'a') as i16 + 1
            } else {
                (byte - b'A') as i16 + 27
            }
        })
        .sum::<i16>()
}

fn part_two(input: &str) -> i16 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let common_type = lines[0]
                .chars()
                .find(|c| lines[1].contains(*c) && lines[2].contains(*c))
                .unwrap();

            let byte = common_type as u8;

            if byte >= b'a' {
                (byte - b'a') as i16 + 1
            } else {
                (byte - b'A') as i16 + 27
            }
        })
        .sum::<i16>()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 157);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 70);
    }
}
