fn parse(input: &str) -> Vec<(u8, u8, u8, u8)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let ((ll, lu), (rl, ru)) = (
                left.split_once('-').unwrap(),
                right.split_once('-').unwrap(),
            );
            (
                ll.parse::<u8>().unwrap(),
                lu.parse::<u8>().unwrap(),
                rl.parse::<u8>().unwrap(),
                ru.parse::<u8>().unwrap(),
            )
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(ll, lu, rl, ru)| (ll >= rl && lu <= ru) || (ll <= rl && lu >= ru))
        .count()
}

fn part_two(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(ll, lu, rl, ru)| ll <= ru && rl <= lu)
        .count()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 2);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 4);
    }
}
