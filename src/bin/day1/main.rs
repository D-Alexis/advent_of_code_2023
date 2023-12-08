use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");

    let start = Instant::now();
    day1(input);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    day2(input);
    println!("day2 took : {:?}", start.elapsed());
}

fn day1(input: &str) {
    let sum = input
        .lines()
        .map(|line| {
            let mut first = '1';
            let mut second = '1';
            for cur_char in line.chars() {
                if ('0'..='9').contains(&cur_char) {
                    first = cur_char;
                    break;
                }
            }
            for cur_char in line.chars().rev() {
                if ('0'..='9').contains(&cur_char) {
                    second = cur_char;
                    break;
                }
            }
            let mut res = String::from(first);
            res.push(second);
            return res.parse::<i32>().unwrap();
        })
        .sum::<i32>();

    println!("{:?}", sum);
}

fn day2(input: &str) {
    let dict = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let total = input
        .lines()
        .map(|line| find_in_line(line, &dict))
        .sum::<i32>();

    println!("{:?}", total);
}
fn find_in_line(line: &str, dict: &HashMap<&str, char>) -> i32 {
    let mut my_vec: Vec<(usize, char)> = Vec::with_capacity(20);
    for c in '0'..='9' {
        if let Some(res) = find_that_char(line, c, false) {
            my_vec.push(res);
        }
        if let Some(res) = find_that_char(line, c, true) {
            my_vec.push(res);
        }
    }

    for c in dict.keys() {
        if let Some(res) = find_that_str(line, c, false, dict) {
            my_vec.push(res);
        }
        if let Some(res) = find_that_str(line, c, true, dict) {
            my_vec.push(res);
        }
    }

    my_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let first = my_vec.first().unwrap().1;
    let second = my_vec.last().unwrap().1;
    let mut res = String::from(first);
    res.push(second);
    return res.parse::<i32>().unwrap();
}
fn find_that_char(line: &str, curr: char, last: bool) -> Option<(usize, char)> {
    if !last {
        if let Some(index) = line.find(curr) {
            Some((index, curr))
        } else {
            None
        }
    } else {
        if let Some(index) = line.rfind(curr) {
            Some((index, curr))
        } else {
            None
        }
    }
}

fn find_that_str(
    line: &str,
    curr: &str,
    last: bool,
    dict: &HashMap<&str, char>,
) -> Option<(usize, char)> {
    if !last {
        if let Some(index) = line.find(curr) {
            Some((index, dict.get(curr).unwrap().clone()))
        } else {
            None
        }
    } else {
        if let Some(index) = line.rfind(curr) {
            Some((index, dict.get(curr).unwrap().clone()))
        } else {
            None
        }
    }
}
