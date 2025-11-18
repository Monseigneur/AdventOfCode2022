use std::collections::{HashMap, VecDeque};

const DAY: usize = 5;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Stacks = HashMap<usize, VecDeque<char>>;

struct Move {
    count: usize,
    start: usize,
    end: usize,
}

impl Move {
    fn new(text: &str) -> Self {
        let mut values = text.split_ascii_whitespace().filter_map(|s| s.parse().ok());

        Self {
            count: values.next().unwrap(),
            start: values.next().unwrap(),
            end: values.next().unwrap(),
        }
    }
}

fn part_1(contents: &str) -> String {
    let (mut stacks, moves) = parse_input(contents);

    moves.into_iter().for_each(|m| apply_move(&mut stacks, &m));

    get_result(&stacks)
}

fn parse_input(contents: &str) -> (Stacks, Vec<Move>) {
    let mut parse_stacks = true;
    let mut stacks = Stacks::new();
    let mut moves = vec![];

    for line in contents.lines() {
        if line.is_empty() {
            parse_stacks = false;
            continue;
        }

        if parse_stacks {
            process_stack_line(&mut stacks, line);
        } else {
            moves.push(Move::new(line));
        }
    }

    (stacks, moves)
}

fn process_stack_line(stacks: &mut Stacks, line: &str) {
    // assuming everything is spaced appropriately, the opening bracket index will be 0, 4, 8, etc
    // (index / 4) + 1 -> column

    let mut column_index = None;
    for (i, c) in line.char_indices() {
        if column_index.is_none() && c == '[' {
            column_index = Some((i / 4) + 1);
            continue;
        }

        if let Some(index) = column_index {
            stacks
                .entry(index)
                .and_modify(|v| v.push_front(c))
                .or_insert(VecDeque::from([c]));
        }

        column_index = None;
    }
}

fn apply_move(stacks: &mut Stacks, m: &Move) {
    for _ in 0..m.count {
        let c = stacks.get_mut(&m.start).unwrap().pop_back().unwrap();
        stacks.entry(m.end).and_modify(|v| v.push_back(c));
    }
}

fn get_result(stacks: &Stacks) -> String {
    let mut result = String::new();

    for i in 1..=stacks.len() {
        result.push(*stacks.get(&i).unwrap().back().unwrap());
    }

    result
}

fn part_2(contents: &str) -> String {
    let (mut stacks, moves) = parse_input(contents);

    moves
        .into_iter()
        .for_each(|m| apply_move_group(&mut stacks, &m));

    get_result(&stacks)
}

fn apply_move_group(stacks: &mut Stacks, m: &Move) {
    let mut crates = VecDeque::new();

    for _ in 0..m.count {
        let c = stacks.get_mut(&m.start).unwrap().pop_back().unwrap();
        crates.push_back(c);
    }

    while let Some(c) = crates.pop_back() {
        stacks.entry(m.end).and_modify(|v| v.push_back(c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), "CMZ");
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), "RNZLFZSJH");
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), "MCD");
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), "CNSFCGJSM");
    }
}
