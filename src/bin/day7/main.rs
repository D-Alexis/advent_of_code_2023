use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

fn main() {
    let input = include_str!("input_test.txt");
    let start = Instant::now();
    let input_info = parse_input(input);
    println!("parsing took : {:?}", start.elapsed());


   let start = Instant::now();
    part1(&input_info);
    println!("day1 took : {:?}", start.elapsed());


   let start = Instant::now();
    part2(input);
    println!("day2 took : {:?}", start.elapsed());


}

fn part1(input : &InputInfo) {

   println!("{:?}", input);
}

fn part2(input :  &str) {
    let mut btree:BTreeSet<Hand> = BTreeSet::new();


  //  btree.iter().for_each(|value| println!("{:?}", value));

   // println!("{:?}", input);
}


fn parse_input(input: &str) -> InputInfo {
    let hands_info = input.lines().map(|line| {
        LineInfo::from_str(line)
    }).collect::<Vec<LineInfo>>();
    InputInfo {
        hands_info
    }
}


#[derive(Debug)]
struct InputInfo {
    hands_info: Vec<LineInfo>
}
#[derive(Debug)]
struct LineInfo {
    bid: u64,
    hand: Hand
}
#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards : Vec<char>,
    high_value: u64,
    hand_type: u64,
}

impl LineInfo {
    fn from_str(line: &str) -> Self {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = Hand::parse_input(hand);
        let bid = bid.parse::<u64>().unwrap();
        LineInfo {
            hand,
            bid
        }
    }
}


impl Hand  {

    fn parse_input(hand: &str) -> Self{
        let hand = hand.chars().collect::<Vec<char>>();
        let hand_type = Self::determine_hand_type(&hand);
        let high_value =  Self::determine_high_value(&hand);
        Hand {
            cards: hand,
            high_value,
            hand_type
        }
    }
    fn determine_hand_type(hand: &Vec<char>) -> u64 {
        let mut map_count : HashMap<char, u64> = HashMap::with_capacity(5);
        hand.iter().for_each(|current| {
            let a = map_count.get(current);
            match a {
                Some(value) => map_count.insert(*current, value +1),
                None => map_count.insert(*current, 1)
            };
        });
        let mut sorted = map_count.values().collect::<Vec<&u64>>();
        //in reverse order
        sorted.sort_by(|a, b| b.cmp(a));
        let mut score = 0;
        for i in 0..sorted.len() {
            match sorted[i] {
                5 => score = score + 2000,
                4 => score = score + 1000,
                3 => score = score + 500,
                2 => score = score + 200,
                &_ => {}
            }
        }
       score
    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type == other.hand_type {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}



impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
       self.compare(other)
    }
}