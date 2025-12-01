use std::collections::HashMap;

const DAY: usize = 7;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug, Default)]
struct Directory {
    size: usize,
    children: Vec<String>,
}

impl Directory {
    fn add_child(&mut self, child_name: &str) {
        self.children.push(child_name.to_string())
    }

    fn add_size(&mut self, additional_size: usize) {
        self.size += additional_size;
    }
}

fn part_1(contents: &str) -> usize {
    let mut file_info = parse_input(contents);

    calculate_sizes(&mut file_info, &"/".to_string());

    find_directories_below_threshold(file_info, 100_000)
}

fn parse_input(contents: &str) -> HashMap<String, Directory> {
    let mut file_info: HashMap<String, Directory> = HashMap::new();

    file_info.insert("/".to_string(), Default::default());

    let mut dir_path = vec![];
    for line in contents.lines() {
        let pieces = line.split_ascii_whitespace().collect::<Vec<_>>();

        if pieces[0] == "$" {
            // Command
            if pieces[1] == "cd" {
                match pieces[2] {
                    ".." => {
                        dir_path.pop();
                    }
                    d => dir_path.push(d),
                };
            }

            continue;
        }

        let parent = dir_path.join("/");

        if let Some(size) = pieces[0].parse::<usize>().ok() {
            file_info.entry(parent).and_modify(|d| d.add_size(size));
        } else {
            let child = parent.clone() + "/" + pieces[1];

            file_info.entry(parent).and_modify(|i| i.add_child(&child));
            file_info.insert(child, Default::default());
        }
    }

    file_info
}

fn calculate_sizes(file_info: &mut HashMap<String, Directory>, current_item: &String) -> usize {
    let children = file_info.get(current_item).unwrap().children.clone();

    let children_sizes: usize = children.iter().map(|i| calculate_sizes(file_info, i)).sum();

    let item = file_info.get_mut(current_item).unwrap();

    item.add_size(children_sizes);

    item.size
}

fn find_directories_below_threshold(
    file_info: HashMap<String, Directory>,
    size_threshold: usize,
) -> usize {
    file_info
        .into_values()
        .filter_map(|item| (item.size <= size_threshold).then_some(item.size))
        .sum()
}

fn part_2(contents: &str) -> usize {
    let mut file_info = parse_input(contents);

    calculate_sizes(&mut file_info, &"/".to_string());

    let current_free_space = 70_000_000 - file_info.get("/").unwrap().size;
    let min_to_delete = 30_000_000 - current_free_space;

    find_directories_closest_to_min(file_info, min_to_delete)
}

fn find_directories_closest_to_min(
    file_info: HashMap<String, Directory>,
    min_threshold: usize,
) -> usize {
    file_info
        .into_values()
        .filter_map(|item| (item.size >= min_threshold).then_some(item.size))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 95437);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1306611);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 24933642);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 13210366);
    }
}
