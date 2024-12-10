use std::fs;

fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let sum: i32 = fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(mult)
        .sum();

    println!("problem one: {}", sum);
}

fn problem_two() {
    let mut active: bool = true;
    let sum: i32 = fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(|line| mult_conditional(line, &mut active))
        .sum();

    println!("problem two: {}", sum);
}

fn mult_conditional(str: &str, active: &mut bool) -> i32 {
    let mut sum = 0;
    let mut start_at = 0;
    while let Some(index) = next_word(&str[start_at..]) {
        start_at += index;
        match &str[start_at..start_at + 3] {
            "mul" => {
                if *active {
                    if let Some(product) = compute(&str[start_at + 4..]) {
                        sum += product;
                    }
                }
            }
            "do(" => *active = true,
            "don" => *active = false,
            _ => {}
        }

        start_at += 4;
    }

    sum
}

fn next_word(str: &str) -> Option<usize> {
    let mul_index = str.find("mul(");
    let dont_index = str.find("don't()");
    let do_index = str.find("do()");

    return [mul_index, dont_index, do_index]
        .iter()
        .filter_map(|index| *index)
        .min();
}

fn mult(str: &str) -> i32 {
    let mut sum = 0;
    let mut start_at = 0;
    while let Some(index) = str[start_at..].find("mul(") {
        start_at += index + 4;

        let Some(product) = compute(&str[start_at..]) else {
            continue;
        };

        sum += product;
    }

    sum
}

fn compute(chunk: &str) -> Option<i32> {
    let (left, right) = chunk.split_once(',')?;
    let left = left.parse::<i32>().ok()?;
    let (right, _) = right.split_once(')')?;
    let right = right.parse::<i32>().ok()?;

    Some(left * right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_matches() {
        let value = mult("mul(20,20)mul(10,10)mult");
        assert_eq!(value, 500);
    }

    #[test]
    fn it_matches_with_conditional() {
        let mut active: bool = true;

        let value = mult_conditional(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            &mut active,
        );
        assert_eq!(value, 48);
    }
}
