use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = InputInfo::parse_input(input);
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
    day1(&input_info);
    println!("day1 took : {:?}", start.elapsed());

   let start = Instant::now();
    day2(&input_info);
    println!("day2 took : {:?}", start.elapsed());


}

fn day1(input_info : &InputInfo) {
    let res = input_info.lines_info.iter().map(|line| {
        let mut score = 0;
        line.numbers_info.numbers.iter().for_each(|num| {
            if line.numbers_info.winning_numbers.contains(num) {
                score = match score {
                    0 => 1,
                    _ => score * 2
                }
            };
        });
        score
    }).sum::<u32>();
    println!("{res}")
}

fn day2(input_info : &InputInfo) {
    let mut card_stack : HashMap<u32, u32> = HashMap::with_capacity(220);
    for i in 1..input_info.lines_info.len()+1 {
        card_stack.insert(i as u32,1);
    }
    let mut total = card_stack.len() as u32;
    input_info.lines_info.iter().for_each(|line| {
        let mut score = 0;
        let card_num = line.card_nb;

        let card_same_num = card_stack.get(&card_num).unwrap().clone();
        line.numbers_info.numbers.iter().for_each(|num| {
            if line.numbers_info.winning_numbers.contains(num) {
                score = score + 1;
            };
        });
        if score > 0 {
            for gained_card in (card_num+1)..=(card_num +score) {
                if gained_card <= card_stack.len()  as u32 {
                    card_stack.insert(gained_card, card_stack.get(&gained_card).unwrap() + card_same_num);
                    total = total + card_same_num;
                }

            }
        }

    });
    println!("{total}");
}


#[derive(Debug)]
struct InputInfo {
    lines_info: Vec<LineInfo>
}
#[derive(Debug)]
struct LineInfo {
    card_nb: u32,
    numbers_info: NumbersInfo
}
#[derive(Debug)]
struct NumbersInfo {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl InputInfo {
    pub fn parse_input(input : &str) -> Self {
        let mut lines_info : Vec<LineInfo> = Vec::with_capacity(220);
        input.lines().for_each(|line| lines_info.push(LineInfo::parse_line(line)));
        InputInfo {
            lines_info
        }
    }
}
impl LineInfo {
    pub fn parse_line(line : &str) -> Self {
        let (games, numbers_list) = line.split_once(": ").unwrap();

        let numbers_info = NumbersInfo::parse_numbers(numbers_list.trim());

        let temp = games.split_once(" ").unwrap().1.trim();
        let card_nb =temp.parse::<u32>().unwrap();
        LineInfo {
            card_nb,
            numbers_info
        }
    }
}

impl NumbersInfo {
    pub fn parse_numbers(all_numbers: &str) -> Self{
        let cleaned = all_numbers.replace("  ", " ");
        let (winning, current_numbers) = cleaned.split_once(" | ").unwrap();

        let winning_numbers: Vec<u32> = str_to_vec_u32(winning);
        let numbers : Vec<u32> = str_to_vec_u32(current_numbers);

        NumbersInfo {
            winning_numbers,
            numbers,
        }
    }
}
fn str_to_vec_u32(numbers : &str) -> Vec<u32> {
    numbers.split(" ").map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}