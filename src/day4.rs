const DAY: usize = 4;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(text: &str) -> Self {
        let mut pieces = text.split('-').map(|x| x.parse::<usize>().unwrap());

        Self {
            start: pieces.next().unwrap(),
            end: pieces.next().unwrap(),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        // [a  {c  b]  d}
        // {c  [a  d}  b]
        let self_first = self.start <= other.start && self.end >= other.start;
        let other_first = other.start <= self.start && other.end >= self.start;

        self_first || other_first
    }
}

fn part_1(contents: &str) -> usize {
    let pairs = parse_input(contents);

    pairs
        .into_iter()
        .filter(|(first, second)| check_pair_contains(first, second))
        .count()
}

fn parse_input(contents: &str) -> Vec<(Range, Range)> {
    contents
        .lines()
        .map(|line| {
            let mut pieces = line.split(',');

            (
                Range::new(pieces.next().unwrap()),
                Range::new(pieces.next().unwrap()),
            )
        })
        .collect()
}

fn check_pair_contains(first: &Range, second: &Range) -> bool {
    first.contains(second) || second.contains(first)
}

fn part_2(contents: &str) -> usize {
    let pairs = parse_input(contents);

    pairs
        .into_iter()
        .filter(|(first, second)| check_pair_overlaps(first, second))
        .count()
}

fn check_pair_overlaps(first: &Range, second: &Range) -> bool {
    first.overlaps(second) || second.overlaps(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 2);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 490);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 4);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 921);
    }
}
