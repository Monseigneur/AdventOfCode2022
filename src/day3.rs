use std::collections::{HashSet, btree_set::Intersection};

const DAY: usize = 3;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Compartment = HashSet<char>;

fn part_1(contents: &str) -> usize {
    let rucksacks = parse_input(contents);

    rucksacks
        .into_iter()
        .map(|(first, second)| calculate_priority(&first, &second))
        .sum()
}

fn parse_input(contents: &str) -> Vec<(Compartment, Compartment)> {
    contents
        .lines()
        .map(|line| {
            let len = line.len() / 2;
            let first_half = line.chars().take(len);
            let second_half = line.chars().skip(len);

            (
                HashSet::from_iter(first_half),
                HashSet::from_iter(second_half),
            )
        })
        .collect()
}

fn get_item_priority(item: char) -> usize {
    match item {
        n @ 'a'..='z' => (n as usize - 'a' as usize) + 1,
        n @ 'A'..='Z' => (n as usize - 'A' as usize) + 27,
        _ => unreachable!(),
    }
}

fn calculate_priority(first: &Compartment, second: &Compartment) -> usize {
    let intersection = first.intersection(&second);

    intersection
        .into_iter()
        .map(|&item| get_item_priority(item))
        .sum()
}

fn part_2(contents: &str) -> usize {
    let rucksacks = parse_input_2(contents);

    let mut result = 0;

    let mut it = rucksacks.into_iter();

    while let Some(first) = it.next() {
        let second = it.next().unwrap();
        let third = it.next().unwrap();

        result += find_group_priority(&first, &second, &third);
    }

    result
}

fn parse_input_2(contents: &str) -> Vec<Compartment> {
    contents
        .lines()
        .map(|line| HashSet::from_iter(line.chars()))
        .collect()
}

fn find_group_priority(first: &Compartment, second: &Compartment, third: &Compartment) -> usize {
    let intersection = first
        .intersection(&second)
        .cloned()
        .collect::<HashSet<char>>();
    let second_intersection = intersection.intersection(&third);

    second_intersection
        .into_iter()
        .map(|&item| get_item_priority(item))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 157);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 7701);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 70);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 2644);
    }
}
