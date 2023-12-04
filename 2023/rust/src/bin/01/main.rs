fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
            let first = numbers.next().expect("should be a number");

            match numbers.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");

            let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
            let first = numbers.next().expect("should be a number");

            match numbers.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_EXAMPLE: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SECOND_EXAMPLE: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(FIRST_EXAMPLE), 77);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(SECOND_EXAMPLE), 99);
    }
}
