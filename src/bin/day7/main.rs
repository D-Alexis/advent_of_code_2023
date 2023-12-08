use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = parse_input(input);
    println!("parsing took : {:?}", start.elapsed());


   let start = Instant::now();
    part(&input_info);
    println!("day1 took : {:?}", start.elapsed());



}

fn part(input : &InputInfo) {
    // Not part 1 anymore
    let mut btree:BTreeSet<&Hand> = BTreeSet::new();
    input.line_info.iter().for_each(|hand| {
        btree.insert(hand);
    });

    let mut total = 0;
    for (i, hand ) in btree.iter().enumerate() {
        total = total + hand.bid * (i+1);
    }
    println!("{total}")
}



fn parse_input(input: &str) -> InputInfo {
    let line_info = input.lines().map(|line| {
        Hand::parse_input(line)
    }).collect::<Vec<Hand>>();
    InputInfo {
        line_info
    }
}


#[derive(Debug)]
struct InputInfo {
    line_info: Vec<Hand>
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
struct Hand {
    bid: usize,
    cards : Vec<usize>,
    high_value: usize,
    hand_type: usize,
}

impl Hand  {

    fn parse_input(line : &str) -> Self{
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let hand = hand.chars().map(|current| Self::change_to_usize(current)).collect::<Vec<usize>>();
        let hand_type = Self::determine_hand_type(&hand);
        let high_value =  Self::determine_high_value(&hand);
        Hand {
            bid,
            cards: hand,
            high_value,
            hand_type
        }
    }
    fn change_to_usize(current: char) -> usize {
        match current {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("wtf is this shit")
        }
    }
    fn determine_hand_type(hand: &Vec<usize>) -> usize {
        let mut map_count : HashMap<usize, usize> = HashMap::with_capacity(5);
        map_count.insert(400, 0);
        hand.iter().for_each(|current| {
            let a = map_count.get(current);
            match a {
                Some(value) => map_count.insert(*current, value +1),
                None => map_count.insert(*current, 1)
            };
        });
        let jokers = if let Some(count) = map_count.get(&0usize) {
            *count
        } else {
            0
        };
        map_count.remove(&0usize);
        let mut sorted = map_count.values().collect::<Vec<&usize>>();
        //in reverse order
        sorted.sort_by(|a, b| b.cmp(a));
        let mut score = 0;
            match sorted[0] + jokers {
                5 => score = score + 2000,
                4 => score = score + 1000,
                3 if *sorted[1] == 2 => score = score + 700,
                3 => score = score + 500,
                2  if *sorted[1] == 2 => score = score + 300,
                2 => score = score + 200,
                _ => {}
            }
       score
    }

    fn determine_high_value(hand: &Vec<usize>) -> usize {
        return
            hand[0] * 100_000_000 +
            hand[1] * 1_000_000 +
            hand[2] * 10_000 +
            hand[3] * 100 +
            hand[4] * 1

    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type == other.hand_type {
            if self.high_value > other.high_value {
                Ordering::Greater
            } else if self.high_value < other.high_value {
                Ordering::Less
            } else {
                Ordering::Equal
            }
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