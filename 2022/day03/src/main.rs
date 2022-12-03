fn part_one(input: &str) -> u32 {
    0
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

    const EXAMPLE: &str = "";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 0);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
