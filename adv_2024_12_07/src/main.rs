use std::fs;

fn main() {
    first();
    second();
}

fn first() {
    let mut result = 0;

    for line in fs::read_to_string("data.txt").unwrap().lines() {
        let d: Result<Vec<i64>, _> = line
            .split_terminator(&[':', ' '][..])
            .filter(|item| !item.is_empty())
            .map(str::parse)
            .collect();

        let data = d.unwrap();
        let n = data.len() - 2;
        let cur = String::new();

        for op in operators(n, cur, false) {
            if evaluate(&data[1..], op, data[0]) {
                result += data[0];
                break;
            }
        }
    }

    println!("{}", result);
}

fn second() {
    let mut result = 0;

    for line in fs::read_to_string("data.txt").unwrap().lines() {
        let d: Result<Vec<i64>, _> = line
            .split_terminator(&[':', ' '][..])
            .filter(|item| !item.is_empty())
            .map(str::parse)
            .collect();

        let data = d.unwrap();
        let n = data.len() - 2;
        let cur = String::new();

        for op in operators(n, cur, true) {
            if evaluate(&data[1..], op, data[0]) {
                result += data[0];
                break;
            }
        }
    }

    println!("{}", result);
}

fn evaluate(data: &[i64], op: String, exp: i64) -> bool {
    let queue: Vec<char> = op.chars().collect();
    let mut data = data.to_vec();

    let mut index = 0;

    while index < queue.len() {
        match queue[index] {
            '+' => {
                data[index + 1] = apply(data[index], data[index + 1], |x, y| x + y);
            }
            '*' => {
                data[index + 1] = apply(data[index], data[index + 1], |x, y| x * y);
            }
            '|' => {
                data[index + 1] = format!("{}{}", data[index], data[index + 1])
                    .parse()
                    .unwrap();
            }
            _ => {}
        }

        data[index] = 0;
        index += 1;
    }

    let mut sum = data[0];
    index = 1;
    while index < data.len() {
        sum = apply(sum, data[index], |x, y| x + y);
        index += 1;
    }
    sum == exp
}

fn apply<T, F>(a: T, b: T, operation: F) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy,
    F: Fn(T, T) -> T,
{
    operation(a, b)
}

fn operators(size: usize, curr: String, add_concat: bool) -> Vec<String> {
    if size == curr.len() {
        return vec![curr];
    }

    let mut opers = Vec::new();

    let chars: &[char] = if add_concat {
        &['+', '*', '|']
    } else {
        &['+', '*']
    };

    for c in chars {
        let data = format!("{}{}", curr, c);
        let mut next_res = operators(size, data, add_concat);
        opers.append(&mut next_res);
    }

    opers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators_works() {
        let n = 2;
        let cur = String::new();

        let result = operators(n, cur, false);
        assert_eq!(result, ["++", "+*", "*+", "**"]);
    }

    #[test]
    fn apply_works() {
        let sum = apply(1, 2, |x, y| x + y);
        let mul = apply(1, 2, |x, y| x * y);

        assert_eq!(sum, 3);
        assert_eq!(mul, 2);
    }

    #[test]
    fn it_works() {
        first();
    }
}
