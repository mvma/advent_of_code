use std::{collections::HashSet, fs};

fn main() {
    let mut data = parse();
    first(&mut data);
    second(&mut data);
}

struct Data {
    rules: HashSet<(u64, u64)>,
    updates: Vec<Vec<u64>>,
    messy: Vec<Vec<u64>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            rules: HashSet::new(),
            updates: vec![],
            messy: vec![],
        }
    }
}

fn parse() -> Data {
    let file_content = fs::read_to_string("data.txt").expect("Could not read the input data");

    let blocks: Vec<&str> = file_content.split("\n\n").collect();

    let mut data = Data::new();

    for block in blocks {
        if block.contains('|') {
            for line in block.lines() {
                if let Some((left, right)) = line.split_once('|') {
                    match (left.trim().parse::<u64>(), right.trim().parse::<u64>()) {
                        (Ok(left_value), Ok(right_value)) => {
                            data.rules.insert((left_value, right_value));
                        }
                        _ => {
                            panic!("Could not parse pipe separated numbers into tuple.");
                        }
                    }
                }
            }
        }

        if block.contains(',') {
            for line in block.lines() {
                let update: Result<Vec<u64>, _> = line
                    .split(',')
                    .map(|item| item.trim().parse::<u64>())
                    .collect();

                match update {
                    Ok(update) => {
                        data.updates.push(update);
                    }
                    _ => {
                        panic!("Could not parse comma separated numbers into vec.");
                    }
                }
            }
        }
    }

    data
}

fn first(data: &mut Data) {
    let mut total = 0;
    for updates in &data.updates {
        if in_order(updates, &data.rules) {
            total += updates[updates.len() / 2];
        }
    }

    println!("first={}", total);
}

fn second(data: &mut Data) {
    for updates in &mut data.updates {
        if !in_order(updates, &data.rules) {
            organize(updates, &data.rules);
            data.messy.push(updates.to_vec());
        }
    }

    let mut total = 0;
    for updates in &mut data.messy {
        total += updates[updates.len() / 2];
    }
    println!("second={}", total);
}

fn organize(messy: &mut Vec<u64>, rules: &HashSet<(u64, u64)>) {
    let mut index = 0;
    while index < messy.len() - 1 {
        let span = &messy[index..=index + 1];
        if !in_order(span, rules) {
            messy.swap(index, index + 1);

            if index > 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }
    }
}

fn in_order(updates: &[u64], rules: &HashSet<(u64, u64)>) -> bool {
    for (left, right) in rules {
        let left_index = updates.iter().position(|item| item == left);
        let right_index = updates.iter().position(|item| item == right);

        match ((left_index), (right_index)) {
            (Some(left_value), Some(right_value)) => {
                if left_value > right_value {
                    return false;
                }
            }
            (None, None) => continue,
            (None, Some(_)) => continue,
            (Some(_), None) => continue,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> HashSet<(u64, u64)> {
        vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn is_messy() {
        let updates = vec![61, 13, 29];
        let rules = get_rules();

        assert_eq!(in_order(&updates, &rules), false);
    }

    #[test]
    fn is_fine() {
        let updates = vec![75, 47, 61, 53, 29];
        let rules = get_rules();

        assert_eq!(in_order(&updates, &rules), true);
    }
}
