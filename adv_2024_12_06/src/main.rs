use std::{collections::HashSet, fs, time::Instant};

fn main() {
    let start = Instant::now();

    let data = first(parse());

    let first_duration = start.elapsed();
    println!("First function executed in: {:?}", first_duration);

    let start_second_v2 = Instant::now();
    second_v2(data);

    let second_v2_duration = start_second_v2.elapsed();
    println!("Second_v2 function executed in: {:?}", second_v2_duration);
}

#[derive(Debug, Clone)]
struct Data {
    map: Vec<Vec<char>>,
    guard_row: i64,
    guard_col: i64,
    visited_positions: HashSet<(i64, i64)>,
}

impl Data {
    pub fn new(map: Vec<Vec<char>>, guard_row: i64, guard_col: i64) -> Self {
        Self {
            map,
            guard_row,
            guard_col,
            visited_positions: HashSet::new(),
        }
    }
}

fn parse() -> Data {
    let (mut guard_row, mut guard_col) = (0, 0);
    let mut grid = vec![];

    let content = fs::read_to_string("data.txt").expect("Could not read the file content");
    for (row_idx, line) in content.lines().enumerate() {
        let mut row = vec![];

        for (col_idx, c) in line.chars().enumerate() {
            if c == '^' {
                guard_row = row_idx as i64;
                guard_col = col_idx as i64;
            }
            row.push(c);
        }
        grid.push(row);
    }

    Data::new(grid, guard_row, guard_col)
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn first(mut data: Data) -> Data {
    let mut direction = 0;
    let rows = data.map.len() as i64 - 1;
    let cols = data.map[0].len() as i64 - 1;
    let (mut guard_row, mut guard_col) = (data.guard_row, data.guard_col);

    while guard_col > 0 && guard_col < cols && guard_row > 0 && guard_row < rows {
        let (drow, dcol) = DIRECTIONS[direction];
        let next_guard_row = guard_row + drow;
        let next_guard_col = guard_col + dcol;

        let cell = data.map[next_guard_row as usize][next_guard_col as usize];

        if cell == '#' {
            direction = (direction + 1) % 4;
        } else {
            guard_row = next_guard_row;
            guard_col = next_guard_col;
            data.visited_positions.insert((guard_row, guard_col));
        }
    }

    println!("Visited distinct positions: {}", data.visited_positions.len());
    data
}

fn second_v2(data: Data) {
    let mut total_loops = 0;

    let valid_positions: Vec<(i64, i64)> = data
        .visited_positions
        .iter()
        .filter(|&&(row, col)| {
            (row, col) != (data.guard_row, data.guard_col)
                && data.map[row as usize][col as usize] != '#'
        })
        .cloned()
        .collect();

    for (row, col) in valid_positions {
        let mut temp_data = data.clone();

        temp_data.map[row as usize][col as usize] = '#';

        if has_loop(temp_data) {
            total_loops += 1;
        }
    }

    println!("Total loop-causing positions: {}", total_loops);
}

fn has_loop(mut data: Data) -> bool {
    let mut state_set = HashSet::new();
    let mut direction = 0;
    let rows = data.map.len() as i64 - 1;
    let cols = data.map[0].len() as i64 - 1;

    while data.guard_col > 0 && data.guard_col < cols && data.guard_row > 0 && data.guard_row < rows {
        let current_state = (data.guard_row, data.guard_col, direction);
        if state_set.contains(&current_state) {
            return true;
        }
        state_set.insert(current_state);

        let (drow, dcol) = DIRECTIONS[direction];
        let next_guard_row = data.guard_row + drow;
        let next_guard_col = data.guard_col + dcol;

        let cell = data.map[next_guard_row as usize][next_guard_col as usize];

        if cell == '#' {
            direction = (direction + 1) % 4;
        } else {
            data.guard_row = next_guard_row;
            data.guard_col = next_guard_col;
        }
    }
    false
}