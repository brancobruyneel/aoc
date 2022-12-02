use std::collections::BinaryHeap;

fn find_most_calories(inventory: &str) -> u32 {
    inventory
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn find_most_calories_top_three(inventory: &str) -> u32 {
    let mut heap = inventory
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<BinaryHeap<u32>>();

    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

fn main() {
    let inventory = include_str!("../input.txt");

    println!("Part one: {}", find_most_calories(inventory));

    println!("Part two: {}", find_most_calories_top_three(inventory));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventory() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
    }

    #[test]
    fn part_one() {
        let most_calories = find_most_calories(inventory());

        assert_eq!(most_calories, 24000);
    }

    #[test]
    fn part_two() {
        let most_top_three_calories = find_most_calories_top_three(inventory());

        assert_eq!(most_top_three_calories, 45000);
    }
}
