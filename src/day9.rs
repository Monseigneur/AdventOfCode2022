use std::collections::HashSet;

const DAY: usize = 9;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(text: &str) -> Self {
        match text {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn step_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn delta(&self, other: &Point) -> (isize, isize) {
        (self.x - other.x, self.y - other.y)
    }
}

fn part_1(contents: &str) -> usize {
    let moves = parse_input(contents);

    simulate_rope(moves, 2)
}

fn parse_input(contents: &str) -> Vec<(Direction, usize)> {
    contents
        .lines()
        .map(|line| {
            let mut pieces = line.split_ascii_whitespace();

            (
                Direction::new(pieces.next().unwrap()),
                pieces.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn adjust_tail(head: &Point, tail: &mut Point) -> bool {
    let (x_delta, y_delta) = head.delta(&tail);

    if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
        // Close enough, tail doesn't need to move.
        return false;
    }

    if x_delta < 0 {
        tail.step_direction(&Direction::Left);
    } else if x_delta > 0 {
        tail.step_direction(&Direction::Right);
    }

    if y_delta < 0 {
        tail.step_direction(&Direction::Down);
    } else if y_delta > 0 {
        tail.step_direction(&Direction::Up);
    }

    true
}

fn part_2(contents: &str) -> usize {
    let moves = parse_input(contents);

    simulate_rope(moves, 10)
}

fn simulate_rope(moves: Vec<(Direction, usize)>, num_nodes: usize) -> usize {
    let mut tail_locations = HashSet::new();

    let mut nodes: Vec<Point> = vec![Default::default(); num_nodes];

    tail_locations.insert(Default::default());

    for (direction, dist) in moves {
        for _ in 0..dist {
            nodes[0].step_direction(&direction);

            let mut moved_tail = true;
            for i in 1..nodes.len() {
                let current_head = nodes[i - 1];

                if !adjust_tail(&current_head, &mut nodes[i]) {
                    moved_tail = false;
                    break;
                }
            }

            if moved_tail {
                tail_locations.insert(*nodes.last().unwrap());
            }
        }
    }

    tail_locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 13);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 6486);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 1);
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 36);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 2678);
    }
}
