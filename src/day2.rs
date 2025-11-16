const DAY: usize = 2;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn new(text: &str) -> Self {
        match text {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

fn part_1(contents: &str) -> usize {
    let moves = parse_input(contents);

    moves.into_iter().map(|(a, b)| calculate_score(a, b)).sum()
}

fn parse_input(contents: &str) -> Vec<(Move, Move)> {
    contents
        .lines()
        .map(|line| {
            let mut pieces = line.split_ascii_whitespace();
            (
                Move::new(pieces.next().unwrap()),
                Move::new(pieces.next().unwrap()),
            )
        })
        .collect()
}

// Score is points for selected shape (Rock = 1, Paper = 2, Scissors = 3) and
// outcome (lost = 0, tie = 3, win = 6)
fn calculate_score(opponent_move: Move, our_move: Move) -> usize {
    let shape_points = match our_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome_points = match (opponent_move, our_move) {
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        _ => 3,
    };

    shape_points + outcome_points
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Tie,
    Win,
}

impl Outcome {
    fn new(text: &str) -> Self {
        match text {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn part_2(contents: &str) -> usize {
    let moves = parse_input_2(contents);

    moves
        .into_iter()
        .map(|(opponents_move, outcome)| {
            let our_move = determine_move(opponents_move, outcome);

            calculate_score(opponents_move, our_move)
        })
        .sum()
}

fn parse_input_2(contents: &str) -> Vec<(Move, Outcome)> {
    contents
        .lines()
        .map(|line| {
            let mut pieces = line.split_ascii_whitespace();
            (
                Move::new(pieces.next().unwrap()),
                Outcome::new(pieces.next().unwrap()),
            )
        })
        .collect()
}

fn determine_move(opponent_move: Move, outcome: Outcome) -> Move {
    match (opponent_move, outcome) {
        (om, Outcome::Tie) => om,
        (Move::Rock, Outcome::Loss) => Move::Scissors,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Scissors, Outcome::Loss) => Move::Paper,
        (Move::Scissors, Outcome::Win) => Move::Rock,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 15);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 10595);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 12);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 9541);
    }
}
