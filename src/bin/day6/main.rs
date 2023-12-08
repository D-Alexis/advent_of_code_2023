use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = parse_input(input);
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
    part1(&input_info);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2_but_smart(input);
    println!("day2 took : {:?}", start.elapsed());
}

fn part1(input: &Vec<(u64, u64)>) {
    println!("{:?}", input);

    let a = input
        .iter()
        .map(|(time_max, record)| {
            let mut number_way_win = 0;
            for i in 0..=*time_max {
                if get_dist(i, *time_max) > *record {
                    number_way_win = number_way_win + 1;
                }
            }
            //return number of way to win
            number_way_win
        })
        .fold(1, |acc, i| acc * i);
    println!("{a}");
}

fn get_dist(time: u64, time_max: u64) -> u64 {
    return time * (time_max - time);
}

fn part2(input: &str) {
    let (time, dest) = input.split_once("\r\n").unwrap();
    let time = str_to_u64(time);
    let dest = str_to_u64(dest);
    println!("{time} and {dest}");

    let mut number_way_win = 0;
    for i in 0..=time {
        if get_dist(i, time) > dest {
            number_way_win = number_way_win + 1;
        }
    }
    println!("{number_way_win}");
}

fn part2_but_smart(input: &str) {
    let (time, dest) = input.split_once("\r\n").unwrap();
    let time = str_to_u64(time) as usize;
    let dest = str_to_u64(dest) as usize;
    println!("{time} and {dest}");

    let x1 = ((0 - time) + f32::sqrt((time * time - 4 * dest) as f32) as usize) / 2;
    let x2 = ((0 - time) - f32::sqrt((time * time - 4 * dest) as f32) as usize) / 2;
    println!("{x1} and {x2}");
    println!("{}", x1 - x2);
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let (times_max, dests_max) = input.split_once("\r\n").unwrap();
    let times = str_to_vec_u64(times_max);
    let dests = str_to_vec_u64(dests_max);
    let mut res = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        res.push((times[i], dests[i]));
    }
    res
}
fn str_to_vec_u64(numbers: &str) -> Vec<u64> {
    numbers
        .split(",")
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
fn str_to_u64(numbers: &str) -> u64 {
    numbers
        .split(",")
        .fold(String::new(), |mut res, i| {
            res.push_str(i);
            res
        })
        .parse::<u64>()
        .unwrap()
}
