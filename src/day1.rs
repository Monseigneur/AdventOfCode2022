const DAY: usize = 1;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let calorie_counts = get_calorie_counts(contents);

    calorie_counts.into_iter().max().unwrap()
}

fn get_calorie_counts(contents: &str) -> Vec<usize> {
    let mut calorie_counts = vec![];

    let mut count = 0;
    for line in contents.lines() {
        if line.is_empty() {
            calorie_counts.push(count);
            count = 0;

            continue;
        }

        let calories = line.parse::<usize>().unwrap();
        count += calories;
    }

    calorie_counts.push(count);

    calorie_counts
}

fn part_2(contents: &str) -> usize {
    let mut calorie_counts = get_calorie_counts(contents);

    calorie_counts.sort_by(|a, b| b.cmp(a));

    calorie_counts.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 24000);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 70369);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 45000);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 203002);
    }
}
