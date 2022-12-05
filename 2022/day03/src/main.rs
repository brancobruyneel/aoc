fn part_one(input: &str) -> i16 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            left.chars()
                .filter(|l| right.contains(*l))
                .map(|l| {
                    let byte = l as u8;

                    if byte >= b'a' {
                        (byte - b'a') as i16 + 1
                    } else {
                        (byte - b'A') as i16 + 27
                    }
                })
                .next()
                .unwrap()
        })
        .sum::<i16>()
}

fn part_two(input: &str) -> u32 {
    0
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
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
