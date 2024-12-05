use std::{borrow::Borrow, fs, vec};

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
            match item.parse::<u32>() {
                Ok(value) => row.data.push(value),
                Err(_) => panic!("Failed to parse '{}' as a number", item),
            }
        }

        self.rows.push(row);
    }

    pub fn problem_one(&mut self) {
        for row in &mut self.rows {
            if row.is_safe() {
                self.total_safe += 1;
            }
        }
    }

    pub fn problem_two(&mut self) {
        for row in &mut self.rows {
            if row.is_safe_with_skip_level() {
                self.total_safe_with_skip += 1;
            }
        }
    }
}

#[derive(Clone)]
struct Row {
    data: Vec<u32>,
}

impl Row {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn is_safe(&self) -> bool {
        let is_asc = self
            .data
            .first()
            .zip(self.data.get(1))
            .map_or(false, |(first, second)| first < second);

        self.data.windows(2).all(|window| {
            let (prev, curr) = (window[0] as i32, window[1] as i32);

            if !self.to_owned().check(prev, curr, is_asc) {
                return false;
            }

            true
        })
    }

    pub fn is_safe_with_skip_level(&self) -> bool {
        let mut has_been_skipped = false;

        let is_asc = self
            .data
            .first()
            .zip(self.data.get(1))
            .map_or(false, |(first, second)| first < second);

        self.data.windows(3).all(|window| {
            let (prev, curr, next) = (window[0] as i32, window[1] as i32, window[2] as i32);

            let worked_first_time = self.to_owned().check(prev, curr, is_asc);

            if !worked_first_time && has_been_skipped {
                return false;
            }

            if !worked_first_time && !has_been_skipped {
                has_been_skipped = true;

                if !self.to_owned().check(next, prev, is_asc) {
                    return false;
                }
            }

            true
        })
    }

    fn check(self, prev: i32, curr: i32, is_asc: bool) -> bool {
        let diff = curr as i32 - prev as i32;

        if is_asc && diff < 0 {
            return false;
        }
        if !is_asc && diff > 0 {
            return false;
        }

        (1..=3).contains(&diff.abs())
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

        assert_eq!(data.total_safe_with_skip, 541);
    }
}
