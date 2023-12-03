use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let input_parsed = input.lines().map(|line| {
        line.chars().map(|node| NodeInfo::new(node)).collect::<Vec<NodeInfo>>()
    }).collect::<Vec<Vec<NodeInfo>>>();

    let start = Instant::now();
    day1(&input_parsed);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    day2(&input_parsed);
    println!("day2 took : {:?}", start.elapsed());


}

fn day1(input_parsed : &Vec<Vec<NodeInfo>>) {
    let mut sum: u32 = 0;
    // c'est parti les dingz
    for i in 0..input_parsed.len() {
        let mut number_str : Vec<char> = Vec::with_capacity(6);
        let mut in_engine = false;
        for j in 0..input_parsed.len() {
            if input_parsed[i][j].is_number {
                in_engine = in_engine | check_around(input_parsed, i, j);
                number_str.push(input_parsed[i][j].value);
            }

            if input_parsed[i][j].is_dot || input_parsed[i][j].is_symbol || j == input_parsed.len() - 1  {
                // it's a dot  or it's the last char on the line
                if number_str.len() > 0 && in_engine {

                    let number = number_str.iter().collect::<String>().parse::<u32>().unwrap();
                    sum = sum+number;
                }
                number_str.clear();
                in_engine = false;
            }
        }

    }
    println!("{}", sum);
}


fn day2(input_parsed : &Vec<Vec<NodeInfo>>) {
    let mut all_gear : HashMap<(usize,usize),Vec<u32>> = HashMap::with_capacity(50);
    // c'est parti les dingz
    for i in 0..input_parsed.len() {
        let mut number_str : Vec<char> = Vec::with_capacity(6);
        let mut in_gear = false;
        let mut current_gear: Option<(usize,usize)> = None;
        for j in 0..input_parsed.len() {
            if input_parsed[i][j].is_number {
                number_str.push(input_parsed[i][j].value);
               if let Some((i_gear,j_gear)) =  check_around_gear(input_parsed, i, j) {
                   current_gear = Some((i_gear,j_gear));
               }
            }

            if input_parsed[i][j].is_dot || input_parsed[i][j].is_symbol || j == input_parsed.len() - 1  {
                // it's a dot  or it's the last char on the line
                if number_str.len() > 0 {
                    if  let Some(gear) = current_gear {
                        let number = number_str.iter().collect::<String>().parse::<u32>().unwrap();

                        if let Some(map) = all_gear.get_mut(&gear) {
                            map.push(number);
                        } else {
                            all_gear.insert(gear, vec![number]);
                        }
                    }

                }
                number_str.clear();
                current_gear = None;
            }
        }

    }
    let mut sum = 0;
    for (gear, values) in all_gear {
        if values.len() > 1 {
            sum = sum + values.iter().fold(1, |acc, current| acc * current)
        }
    }
    println!("{:?}", sum);
}


fn check_around(input_parsed: &Vec<Vec<NodeInfo>>, i: usize, j: usize) -> bool {
    let max = input_parsed.len();
    // line above
    if i > 0 {
        if input_parsed[i-1][j].is_symbol {
            return true
        }
        if j > 0 && input_parsed[i-1][j-1].is_symbol {
            return true
        }
        if (j < max - 1) && input_parsed[i-1][j+1].is_symbol {
            return true
        }
    }
    // line under
    if i < max - 1 {
        if input_parsed[i+1][j].is_symbol {
            return true
        }
        if j > 0 && input_parsed[i+1][j-1].is_symbol {
            return true
        }
        if (j < max - 1) && input_parsed[i+1][j+1].is_symbol {
            return true
        }
    }
    // just left
    if (j > 0) && (input_parsed[i][j-1].is_symbol) {
        return true
    }

    // just right
    if (j < max - 1) && (input_parsed[i][j+1].is_symbol) {
        return true
    }
    false
}

fn check_around_gear(input_parsed: &Vec<Vec<NodeInfo>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let max = input_parsed.len();
    // line above
    if i > 0 {
        if input_parsed[i-1][j].is_gear {
            return Some((i-1, j))
        }
        if j > 0 && input_parsed[i-1][j-1].is_gear {
            return Some((i-1, j-1))
        }
        if (j < max - 1) && input_parsed[i-1][j+1].is_gear {
            return Some((i-1, j+1))
        }
    }
    // line under
    if i < max - 1 {
        if input_parsed[i+1][j].is_gear {
            return Some((i+1, j))
        }
        if j > 0 && input_parsed[i+1][j-1].is_gear {
            return Some((i+1, j-1))
        }
        if (j < max - 1) && input_parsed[i+1][j+1].is_gear {
            return Some((i+1, j+1))
        }
    }
    // just left
    if (j > 0) && (input_parsed[i][j-1].is_gear) {
        return Some((i, j-1))
    }

    // just right
    if (j < max - 1) && (input_parsed[i][j+1].is_gear) {
        return Some((i, j+1))
    }
    None
}


#[derive(Debug)]
struct NodeInfo {
    value: char,
    is_symbol: bool,
    is_number: bool,
    is_dot: bool,
    is_gear: bool
}

impl NodeInfo {
    pub fn new(node: char) -> Self {
        match node {
            '0'..='9' => NodeInfo {value: node, is_symbol: false, is_number: true, is_dot: false, is_gear : false},
            '.' => NodeInfo {value: node, is_symbol: false, is_number: false, is_dot: true, is_gear : false},
            '*' => NodeInfo {value: node, is_symbol: true, is_number: false, is_dot: false, is_gear : true},
            // If it's not a number nor a dot, it should be a symbol.
            _ => NodeInfo {value: node, is_symbol: true, is_number: false, is_dot: false, is_gear : false},
        }
    }
}
