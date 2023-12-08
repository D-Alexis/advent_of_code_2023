use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = InputInfo::from_parse(input);
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
 //   part1(&input_info);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2(&input_info);
    println!("day1 took : {:?}", start.elapsed());
}
fn part1(input: &InputInfo) {
    // Not part 1 anymore
    let mut a: &str = "AAA";

    let mut found = false;
    let mut step: usize = 0;
    while !found {
        for instr in &input.instructions {
            let (left, right) = input.line_info.get(a).unwrap();
            if instr == &'L' {
                a = &left;
            } else {
                a = &right;
            }

            if a == "ZZZ" {
                found = true;
            }
            step = step + 1;
        }
    }

    println!("{:?}", step);
}

fn part2(input: &InputInfo) {
    let mut found = false;
    let mut step: usize = 0;
    let mut all_starts = input.starts.clone();
    let mut all_vec = Vec::with_capacity(6);
    for mut start in all_starts {
        let mut found = false;
        step = 0;
        while !found {
            for instr in &input.instructions {
                let (left, right) = input.line_info.get(&start).unwrap();
                if instr == &'L' {
                    start = left.clone();
                } else {
                    start = right.clone();
                }
                step = step + 1;
                if start.ends_with("Z") {
                    found = true;
                    break;
                }

            }
        }
        all_vec.push(step);
        println!("what {:?}", step);

    }

    println!("{:?}", all_vec.iter().fold(1usize, |a, b | num_integer::lcm(*b, a)));
}

#[derive(Debug)]
struct InputInfo {
    instructions: Vec<char>,
    starts: Vec<String>,
    line_info: HashMap<String, (String, String)>,
}

impl InputInfo {
    fn from_parse(input: &str) -> Self {
        let (instructions, map_info) = input.split_once("\n\n").unwrap();
        let mut starts: Vec<String> = Vec::with_capacity(200);
        let mut line_info: HashMap<String, (String, String)> = HashMap::with_capacity(790);
        map_info.lines().for_each(|info| {
            let (from, to) = info.split_once(" = (").unwrap();
            let (left, right) = to.split_once(", ").unwrap();
            let right = &right[..right.len() - 1];
            line_info.insert(from.to_string(), (left.to_string(), right.to_string()));
            if from.ends_with('A') {
                starts.push(from.to_string());
            }
        });
        InputInfo {
            instructions: instructions.chars().collect::<Vec<char>>(),
            line_info,
            starts,
        }
    }
}
