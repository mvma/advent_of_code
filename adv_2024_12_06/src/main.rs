use std::{collections::HashSet, fs};

fn main() {
    let data = parse();
    let first = first(data);
    println!("first {}", first);
}

#[derive(Debug)]
struct Data {
    map: Vec<Vec<char>>,
    guard_row: i64,
    guard_col: i64,
}

impl Data {
    pub fn new(map: Vec<Vec<char>>, guard_row: i64, guard_col: i64) -> Self {
        Self {
            map,
            guard_row,
            guard_col,
        }
    }
}

fn parse() -> Data {
    let (mut guard_row, mut guard_col) = (0, 0);
    let mut outer = vec![];

    let content = fs::read_to_string("data.txt").expect("Could not read the file content");
    for (row, line) in content.lines().enumerate() {
        let mut inner = vec![];

        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard_row = row as i64;
                guard_col = col as i64;
            }
            inner.push(c);
        }
        outer.push(inner);
    }

    Data::new(outer, guard_row, guard_col)
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn first(mut data: Data) -> usize {
    let mut distinct_positions = HashSet::new();
    let mut direction: usize = 0;
    let rows = data.map.len() as i64 - 1;
    let cols = data.map[0].len() as i64 - 1;

    while data.guard_col > 0
        && data.guard_col < cols
        && data.guard_row > 0
        && data.guard_row < rows
    {
        let (drow, dcol) = DIRECTIONS[direction];
        let next_guard_row = data.guard_row + drow;
        let next_guard_col = data.guard_col + dcol;

        let char = data.map[next_guard_row as usize][next_guard_col as usize];

        if char == '#' {
            direction = (direction + 1) % 4;
        } else {
            data.guard_row = next_guard_row;
            data.guard_col = next_guard_col;
            distinct_positions.insert((data.guard_row, data.guard_col));
        }
    }

    distinct_positions.len()
}
