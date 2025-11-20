use std::collections::HashSet;

const DAY: usize = 6;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    find_unique_window(contents, 4)
}

fn find_unique_window(text: &str, window_size: usize) -> usize {
    let chars = text.chars().collect::<Vec<char>>();

    for (i, a) in chars.windows(window_size).enumerate() {
        let mut window = HashSet::new();

        for c in a {
            window.insert(c);
        }

        if window.len() == window_size {
            return i + window_size;
        }
    }

    unreachable!()
}

fn part_2(contents: &str) -> usize {
    find_unique_window(contents, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 7);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1034);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 19);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 2472);
    }
}
