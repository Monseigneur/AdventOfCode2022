const DAY: usize = 8;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid<T> = Vec<Vec<T>>;

fn part_1(contents: &str) -> usize {
    let tree_grid = parse_input(contents);

    count_visible_trees(&tree_grid)
}

fn parse_input(contents: &str) -> Grid<usize> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn count_visible_trees(tree_grid: &Grid<usize>) -> usize {
    let mut visibility_grid = vec![vec![false; tree_grid[0].len()]; tree_grid.len()];

    // Prefill the edges
    for r in 0..tree_grid.len() {
        visibility_grid[r][0] = true;
        visibility_grid[r][tree_grid[0].len() - 1] = true;
    }

    for c in 0..tree_grid[0].len() {
        visibility_grid[0][c] = true;
        visibility_grid[tree_grid.len() - 1][c] = true;
    }

    check_rows(tree_grid, &mut visibility_grid);
    check_cols(tree_grid, &mut visibility_grid);

    visibility_grid
        .into_iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}

fn check_rows(tree_grid: &Grid<usize>, visibility_grid: &mut Grid<bool>) {
    for row in 0..tree_grid.len() {
        let last_col = tree_grid[row].len() - 1;

        let mut max_tree = tree_grid[row][0];
        for c in 1..=last_col {
            if tree_grid[row][c] > max_tree {
                visibility_grid[row][c] = true;
                max_tree = tree_grid[row][c];
            }
        }

        max_tree = tree_grid[row][last_col];
        for c in (0..last_col).rev() {
            if tree_grid[row][c] > max_tree {
                visibility_grid[row][c] = true;
                max_tree = tree_grid[row][c];
            }
        }
    }
}

fn check_cols(tree_grid: &Grid<usize>, visibility_grid: &mut Grid<bool>) {
    for col in 0..tree_grid[0].len() {
        let last_row = tree_grid.len() - 1;

        let mut max_tree = tree_grid[0][col];
        for row in 1..=last_row {
            if tree_grid[row][col] > max_tree {
                visibility_grid[row][col] = true;
                max_tree = tree_grid[row][col];
            }
        }

        max_tree = tree_grid[last_row][col];
        for row in (0..last_row).rev() {
            if tree_grid[row][col] > max_tree {
                visibility_grid[row][col] = true;
                max_tree = tree_grid[row][col];
            }
        }
    }
}

fn part_2(contents: &str) -> usize {
    let tree_grid = parse_input(contents);

    tree_grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_data)| {
            row_data
                .iter()
                .enumerate()
                .map(|(col, _)| calculate_scenic_score(&tree_grid, row, col))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap()
}

fn calculate_scenic_score(tree_grid: &Grid<usize>, row: usize, col: usize) -> usize {
    let last_row = tree_grid.len() - 1;
    let last_col = tree_grid[0].len() - 1;

    if row == 0 || row == last_row || col == 0 || col == last_col {
        return 0;
    }

    let height = tree_grid[row][col];

    let mut left_score = 0;
    for r in (0..row).rev() {
        left_score += 1;

        if tree_grid[r][col] >= height {
            break;
        }
    }

    let mut right_score = 0;
    for r in (row + 1)..=last_row {
        right_score += 1;

        if tree_grid[r][col] >= height {
            break;
        }
    }

    let mut up_score = 0;
    for c in (0..col).rev() {
        up_score += 1;

        if tree_grid[row][c] >= height {
            break;
        }
    }

    let mut down_score = 0;
    for c in (col + 1)..=last_col {
        down_score += 1;

        if tree_grid[row][c] >= height {
            break;
        }
    }

    left_score * right_score * up_score * down_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 21);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1779);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 8);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 172224);
    }
}
