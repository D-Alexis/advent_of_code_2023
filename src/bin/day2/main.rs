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

fn day1(input : &InputInfo) {
    let max_red : u32 = 12;
    let max_blue : u32 = 14;
    let max_green : u32 = 13;
    let mut game_sum = 0;
    input.lines_info.iter().for_each(|line_info: &LineInfo| {
        let mut possible = true;
        line_info.handfuls.iter().for_each(|handful: &Handful| {
            if handful.red_nb > max_red || handful.green_nb > max_green || handful.blue_nb > max_blue {
                possible = false;
            }
        });
        if possible {
           game_sum = game_sum + line_info.game_nb;
        }
    });
    println!("{game_sum}")
}

fn day2(input: &InputInfo) {
    let mut total_power = 0;
    input.lines_info.iter().for_each(|line_info: &LineInfo| {
        let mut max_red : u32 = 1;
        let mut max_blue : u32 = 1;
        let mut max_green : u32 = 1;
        line_info.handfuls.iter().for_each(|handful: &Handful| {
            max_red = max_red.max(handful.red_nb);
            max_green = max_green.max(handful.green_nb);
            max_blue = max_blue.max(handful.blue_nb);
        });
        total_power = total_power + (max_red * max_green * max_blue);
    });
    println!("{total_power}");
}


#[derive(Debug)]
struct InputInfo {
    lines_info: Vec<LineInfo>
}
#[derive(Debug)]
struct LineInfo {
    game_nb: u32,
    handfuls: Vec<Handful>
}
#[derive(Debug)]
struct Handful {
    red_nb : u32,
    blue_nb : u32,
    green_nb : u32
}

impl InputInfo {
    pub fn parse_input(input : &str) -> Self {
        let mut lines_info : Vec<LineInfo> = Vec::with_capacity(100);
        input.lines().for_each(|line| lines_info.push(LineInfo::parse_line(line)));
        InputInfo {
            lines_info
        }
    }
}
impl LineInfo {
    pub fn parse_line(line : &str) -> Self {
        let (games, handful_list) = line.split_once(": ").unwrap();

        let mut handful_vec : Vec<Handful> = Vec::with_capacity(7);
        handful_list.split("; ").for_each(|handful| handful_vec.push(Handful::parse_handful(handful)));

        LineInfo {
            game_nb: games.split_once(" ").unwrap().1.parse::<u32>().unwrap(),
            handfuls: handful_vec
        }
    }
}

impl Handful {
    pub fn parse_handful(handful: &str) -> Self{
        let mut res = Handful {
            red_nb :0,
            blue_nb: 0,
            green_nb: 0
        };

        handful.split(", ").for_each(|cubes| {
            let (number_str, color) = cubes.split_once(' ').unwrap();
            let number = number_str.parse::<u32>().unwrap();
            match color {
                "red" => res.red_nb = number,
                "blue" => res.blue_nb = number,
                "green" => res.green_nb = number,
                _ => panic!("wtf is this color bro")
            };
        });
        res
    }
}