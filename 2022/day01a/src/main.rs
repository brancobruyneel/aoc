use std::fs;

pub struct Elf {
    index: usize,
    calories: u32,
}

pub fn find_elf_with_most_calories(inventory: String) -> Elf {
    let elf: (usize, u32) = inventory
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    Elf {
        index: elf.0,
        calories: elf.1,
    }
}

fn main() {
    let inventory = fs::read_to_string("input.txt").expect("could not read file");

    let elf = find_elf_with_most_calories(inventory);

    println!("{}", elf.calories);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example() {
        let inventory = fs::read_to_string("example.txt").expect("could not read file");

        let elf = find_elf_with_most_calories(inventory);

        assert_eq!(elf.index, 3);
        assert_eq!(elf.calories, 24000);
    }
}
