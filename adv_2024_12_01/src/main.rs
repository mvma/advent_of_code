use std::{collections::HashMap, fs};

fn main() {
    let mut solver = Solver::new();
    solver.feed("data.txt");
    solver.problem_one();
    solver.problem_two();
}

struct Solver {
    left: Vec<u64>,
    right: Vec<u64>,
    size: usize,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            size: 0,
        }
    }

    pub fn feed(&mut self, path: &str) {
        let content = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!("Failed to read file '{}': {}", path, err);
        });

        let mut position = 0;
        for item in content.split_whitespace() {
            match item.parse::<u64>() {
                Ok(value) => {
                    if position % 2 == 0 {
                        self.left.push(value);
                    } else {
                        self.right.push(value);
                    }
                    position += 1;
                }
                Err(_) => panic!("Failed to parse '{}' as a number", item),
            }
        }

        self.left
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        self.right
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        self.size = position / 2;
    }

    pub fn problem_one(&mut self) -> u64 {
        let mut sum = 0;
        let mut current = 0;

        while current < self.size {
            sum += self.left[current].abs_diff(self.right[current]);

            current += 1;
        }

        sum
    }

    pub fn problem_two(&mut self) -> u64 {
        let mut sum = 0;
        let mut current = 0;

        let mut map = HashMap::new();

        while current < self.size {
            let item = self.right[current];

            if let Some(count) = map.get_mut(&item) {
                *count += 1;
            } else {
                map.insert(item, 1);
            }

            current += 1;
        }

        current = 0;

        while current < self.size {
            let item = self.left[current];

            if let Some(count) = map.get(&item) {
                sum += item * count;
            }

            current += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_feeds() {
        let mut solver = Solver::new();
        solver.feed("data.txt");

        assert_eq!(solver.size, 1000);
    }

    #[test]
    fn it_solves_problem_one() {
        let mut solver = Solver::new();
        solver.feed("data.txt");
        assert_eq!(solver.problem_one(), 936063);
    }

    #[test]
    fn it_solves_problem_two() {
        let mut solver = Solver::new();
        solver.feed("data.txt");
        assert_eq!(solver.problem_two(), 23150395);
    }
}
