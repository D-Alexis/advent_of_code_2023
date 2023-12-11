use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
  //  part1(input);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2(&input);
    println!("day1 took : {:?}", start.elapsed());
}
fn part1(input: &str) {

    let mut mapped = parse(input);
    let start = mapped.start;
    let mut nodes = &mut mapped.nodes;
    let mut next_nodes : VecDeque<Node>= VecDeque::with_capacity(140);
    let mut max = 0;
    let mut start = nodes[start.0 as usize][(start.1) as usize].clone();

    next_nodes.push_back( start);
     while let Some(current) = next_nodes.pop_front() {
         for cur_dir in &current.directions {
             let  next_node = &mut nodes[(current.pos.0 + cur_dir.dir.0) as usize][(current.pos.1 + cur_dir.dir.1) as usize];

             let a = &next_node.directions;
             for next_dir in a {
                 if cur_dir.can_go_to(next_dir) && (!next_node.visited || next_node.length > current.length + 1) {
                     next_node.length = current.length + 1;
                     max = max.max(next_node.length);
                     next_node.visited = true;
                     next_nodes.push_back(next_node.clone());
                 }
             };

         };
     }
    println!("{:?}", max);
}
fn part2(input: &str) {

    let mut mapped = parse(input);
    let start = mapped.start;
    let mut nodes = &mut mapped.nodes;
    let mut next_nodes : VecDeque<Node>= VecDeque::with_capacity(140);
    let mut max = 0;
    let mut start = nodes[start.0 as usize][(start.1) as usize].clone();

    next_nodes.push_back( start);
    while let Some(current) = next_nodes.pop_front() {
        for cur_dir in &current.directions {
            let  next_node = &mut nodes[(current.pos.0 + cur_dir.dir.0) as usize][(current.pos.1 + cur_dir.dir.1) as usize];

            let a = &next_node.directions;
            for next_dir in a {
                if cur_dir.can_go_to(next_dir) && (!next_node.visited || next_node.length > current.length + 1) {
                    next_node.length = current.length + 1;
                    max = max.max(next_node.length);
                    next_node.visited = true;
                    next_nodes.push_back(next_node.clone());
                }
            };

        };
    }
    let wall = vec!['|','F', '7']; // 'S' in my input is a 'L' otherwise i add it for example
    let mut count = 0;
    for i in 0..nodes.len() {
        let mut inside = false;
        for j in 0..nodes[i].len() {
            let a = &nodes[i][j];
            if a.visited {
                inside ^= wall.contains(&a.car);
            } else {
                if inside {
                    count = count + 1;
                }
            }

        }
    }
    println!("{count}");
}

fn parse(input: &str) -> Map {
    let mut all_mapped : Vec<Vec<Node>> = Vec::with_capacity(140);
    let mut start: (isize, isize) = (0, 0);
    input.lines().for_each(|line| {
        let mut line_map:Vec<Node> = Vec::with_capacity(140);
        line.chars().for_each(|current| {
            let directions = match current {
                '-' => vec![Dir{dir: (0, 1)},Dir{dir: (0, -1)}],
                '|' => vec![Dir{dir: (-1, 0)},Dir{dir: (1, 0)}],
                'L' => vec![Dir{dir: (-1, 0)},Dir{dir: (0, 1)}],
                'J' => vec![Dir{dir: (-1, 0)},Dir{dir: (0, -1)}],
                '7' => vec![Dir{dir: (0, -1)},Dir{dir: (1, 0)}],
                'F' => vec![Dir{dir: (1, 0)},Dir{dir: (0, 1)}],
                '.' => vec![Dir{dir: (0, 0)}],
                'S' => {start = (all_mapped.len() as isize, line_map.len() as isize);vec![Dir{dir: (1, 0 )},Dir{dir: (-1, 0)},Dir{dir: (0, 1)},Dir{dir: (0, -1)}]}, // useless
                _ => panic!("holy shit wtf")
            };
            // vec![Dir{dir: (1, 0 )},Dir{dir: (-1, 0)},Dir{dir: (0, 1)},Dir{dir: (0, -1)}]
            line_map.push(Node {visited: current == 'S' , directions , length: 0, pos: (all_mapped.len() as isize, line_map.len() as isize), car : current});
        });
        all_mapped.push(line_map);
    });
    Map { nodes : all_mapped, start }
}


#[derive(Debug, Clone)]
struct Node {
    visited: bool,
    directions: Vec<Dir>,
    length: isize,
    pos: (isize, isize),
    car: char
}
#[derive(Debug, Clone)]
struct Map {
    nodes: Vec<Vec<Node>>,
    start: (isize, isize)
}
#[derive(Debug, Clone)]
struct Dir {
    dir : (isize, isize)
}

impl Dir {
    fn can_go_to(&self, other: &Self) -> bool {
        if self.dir.0 + other.dir.0 == 0 && self.dir.1 + other.dir.1 == 0 {
            return true
        }
        false
    }
}