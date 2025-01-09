use std::{
    collections::{HashMap, HashSet},
    fs, i64,
};

fn main() {
    first();
    second();
}

fn first() {
    let mut antennas = HashMap::new();

    let binding = fs::read_to_string("data.txt").unwrap();

    let input = binding.lines().collect::<Vec<_>>();

    let total_rows = input.len() as i64;
    let total_cols = input[0].len() as i64;

    for (row_index, line) in input.iter().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            if char.is_alphanumeric() {
                antennas
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push((row_index as i64, col_index as i64));
            }
        }
    }

    println!("{:?}", antennas);

    let mut antinodes: HashSet<(i64, i64)> = HashSet::default();

    for values in antennas.values() {
        for i in 0..values.len() - 1 {
            for j in i + 1..values.len() {
                let (row1, col1) = values[i];
                let (row2, col2) = values[j];

                let rise = row1 - row2;
                let run = col1 - col2;

                let l1 = (row1 + rise, col1 + run);
                let l2 = (row2 - rise, col2 - run);

                if in_bound(l1.0, l1.1, total_rows, total_cols) {
                    antinodes.insert((l1.0, l1.1));
                }

                if in_bound(l2.0, l2.1, total_rows, total_cols) {
                    antinodes.insert((l2.0, l2.1));
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn second() {
    let mut antennas = HashMap::new();

    let binding = fs::read_to_string("data.txt").unwrap();

    let input = binding.lines().collect::<Vec<_>>();

    let total_rows = input.len() as i64;
    let total_cols = input[0].len() as i64;

    for (row_index, line) in input.iter().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            if char.is_alphanumeric() {
                antennas
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push((row_index as i64, col_index as i64));
            }
        }
    }

    println!("{:?}", antennas);

    let mut antinodes: HashSet<(i64, i64)> = HashSet::default();

    for values in antennas.values() {
        for i in 0..values.len() - 1 {
            for j in i + 1..values.len() {
                let (row1, col1) = values[i];
                let (row2, col2) = values[j];

                let rise = row1 - row2;
                let run = col1 - col2;

                let mut row3 = row1;
                let mut col3 = col1;

                while in_bound(row3, col3, total_rows, total_cols) {
                    antinodes.insert((row3, col3));

                    row3 += rise;
                    col3 += run;
                }

                let mut row4 = row2;
                let mut col4 = col2;

                while in_bound(row4, col4, total_rows, total_cols) {
                    antinodes.insert((row4, col4));

                    row4 -= rise;
                    col4 -= run;
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn in_bound(row: i64, col: i64, total_rows: i64, total_cols: i64) -> bool {
    row >= 0 && row < total_rows && col >= 0 && col < total_cols
}
