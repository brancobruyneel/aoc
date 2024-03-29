fn parse(input: &str) -> Vec<(u8, u8, u8, u8)> {

            let ((left_lower, left_upper), (right_lower, right_upper)) = (
                left.split_once('-').unwrap(),
                right.split_once('-').unwrap(),
            );

            (left_lower <= right_lower && left_upper <= right_upper)
                || (left_lower >= right_lower && left_upper >= right_upper)
        })
        .filter(|is_fully_contained| *is_fully_contained)
        .count()
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
<<<<<<< HEAD
2-6,4-8";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 2);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 4);
    }
}
