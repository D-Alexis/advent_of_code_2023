use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
    part1(input);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2(&input);
    println!("day1 took : {:?}", start.elapsed());
}
fn part1(input: &str) {
    let sum = input.lines().map(|line| find_next_num(line))
        .sum::<isize>();
    println!("{sum}");
}

fn find_next_num(line: &str) -> isize {
    let nums = line.split(" ").map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let max = nums.len();
    let mut matrices : Vec<Vec<isize>> = Vec::with_capacity(max+1);

    matrices.push(nums);
    let mut brreak = 0;

    for i in 1..=max {
        let mut new_line :Vec<isize> = Vec::with_capacity(max-i + 1);
        for j in 0..new_line.capacity()-1 {
            new_line.push(matrices[i-1][j+1] - matrices[i-1][j]);

        }
        if new_line.iter().fold(true, |a, b| a & (*b == 0isize)) {
            brreak = brreak + 1;
            break;
        }
        matrices.push(new_line);

    }
    let max = matrices.len();
    for i in 1..matrices.len() {
        let last = matrices[max-1-i].last().unwrap();
        let new = matrices[max-i].last().unwrap() + last;
        matrices[max-1-i].push(new);
      //  println!("{new}");
    }


    *matrices[0].last().unwrap()
}

fn part2(input: &str) {
    let sum = input.lines().map(|line| find_prev_num(line))
        .sum::<isize>();
    println!("{sum}");
}


fn find_prev_num(line: &str) -> isize {
    let nums = line.split(" ").map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let max = nums.len();
    let mut matrices : Vec<Vec<isize>> = Vec::with_capacity(max+1);

    matrices.push(nums);
    let mut brreak = 0;

    for i in 1..=max {
        let mut new_line :Vec<isize> = Vec::with_capacity(max-i + 1);
        for j in 0..new_line.capacity()-1 {
            new_line.push(matrices[i-1][j+1] - matrices[i-1][j]);

        }
        if new_line.iter().fold(true, |a, b| a & (*b == 0isize)) {
            brreak = brreak + 1;
            break;
        }
        matrices.push(new_line);

    }
    let max = matrices.len();
    for i in 1..matrices.len() {
        let first = matrices[max-1-i][0];
        let new = first - matrices[max-i].last().unwrap();
        matrices[max-1-i].push(new);
          println!("{new}");
    }


    *matrices[0].last().unwrap()
}