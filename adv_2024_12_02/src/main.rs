use std::{fs, vec};

fn main() {
    let mut data = feed("data.txt");
    data.problem_one();
    data.problem_two();

    println!("Total safe {}", data.total_safe);
    println!("Total safe with skip {}", data.total_safe_with_skip);
}

struct Data {
    rows: Vec<Row>,
    total_safe: i32,
    total_safe_with_skip: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            rows: vec![],
            total_safe: 0,
            total_safe_with_skip: 0,
        }
    }

    pub fn add(&mut self, line: &str) {
        let mut row = Row::new();
        for item in line.split_whitespace() {
            match item.parse::<i32>() {
                Ok(value) => row.data.push(value),
                Err(_) => panic!("Failed to parse '{}' as a number", item),
            }
        }

        self.rows.push(row);
    }

    pub fn problem_one(&mut self) {
        for row in &mut self.rows {
            if row.check(false) {
                self.total_safe += 1;
            }
        }
    }

    pub fn problem_two(&mut self) {
        for row in &mut self.rows {
            if row.check(true) {
                self.total_safe_with_skip += 1;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Row {
    data: Vec<i32>,
}

impl Row {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn check(&self, retry: bool) -> bool {
        let is_safe = self.is_safe(&self.data);
        match (is_safe, retry) {
            (true, true) => is_safe,
            (true, false) => is_safe,
            (false, true) => {
                let mut index = 0;
                loop {
                    let mut data = self.data.clone();

                    data.remove(index);

                    if self.is_safe(&data) {
                        println!("{:?}", data);
                        return true;
                    }

                    index += 1;

                    if index == self.data.len() {
                        return false;
                    }
                }
            }
            _ => false,
        }
    }

    fn is_safe(&self, vec: &[i32]) -> bool {
        let is_asc = vec[0] <= vec[1];
        vec.windows(2).all(|window| {
            let (prev, curr) = (window[0], window[1]);

            let diff = curr - prev;

            if is_asc && diff < 0 {
                return false;
            }
            if !is_asc && diff > 0 {
                return false;
            }

            (1..=3).contains(&diff.abs())
        })
    }
}

fn feed(path: &str) -> Data {
    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read file '{}': {}", path, err);
    });

    let mut data = Data::new();
    for line in content.lines() {
        data.add(line);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_feeds() {
        let data = feed("data.txt");

        assert_eq!(data.rows.len(), 1000);
    }

    #[test]
    fn it_solves_problem_one() {
        let mut data = feed("data.txt");
        data.problem_one();

        assert_eq!(data.total_safe, 483);
    }

    #[test]
    fn it_solves_problem_two() {
        let mut data = feed("data.txt");
        data.problem_two();

        assert_eq!(data.total_safe_with_skip, 528);
    }
}
