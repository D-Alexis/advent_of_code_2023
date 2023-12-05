use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = InputInfo::parse_input(input);
    println!("parsing took : {:?}", start.elapsed());

  /*  let start = Instant::now();
    day1(&input_info);
    println!("day1 took : {:?}", start.elapsed());
*/

   let start = Instant::now();
    day2(&input_info);
    println!("day2 took : {:?}", start.elapsed());


}

fn day1(input_info : &InputInfo) {
    let mut source = input_info.seeds.clone();

    input_info.map_infos.iter().for_each(|map_info| {
        source = find_all_next(&source, map_info)
    });
    println!("{:?}", source.iter().min().unwrap());
}
fn find_all_next(sources: &Vec<u64>, map_info: &MapInfo) -> Vec<u64>{
    let mut res :Vec<u64> = Vec::with_capacity(sources.len());
    for source in sources {
        let mut dest = source.clone();
        for range_info in &map_info.numbers_info {
            // if we're in a specified range
            if source >= &range_info.source_min && source <= &range_info.source_max {
                dest = range_info.dest_min + (source - range_info.source_min)
            }
        }
        res.push(dest);
    }
    res
}

fn day2(input_info : &InputInfo) {

}

fn get_that_fucking_map(map_infos : Vec<MapInfo>) {
    let mut iter= map_infos.iter();
    let mut res = iter.next().unwrap().numbers_info.clone();

    iter.for_each(|map_info|  {
        map_info.numbers_info.iter().for_each(|range_info| {
            the_what(&mut res, range_info);
        });
    });
}
fn the_what(res: &mut Vec<RangeInfo>, range_info: &RangeInfo) {
    let mut new_range_info: Vec<RangeInfo> = Vec::with_capacity(5);
    res.iter_mut().for_each(|current| {
        //update range
        if current.dest_min >= range_info.source_min && current.dest_min <= range_info.source_min {

        }

        //new_range_info.push(current.clone())
    });

    new_range_info.iter().for_each(|new_range| res.push(new_range.clone()))
}

#[derive(Debug)]
struct InputInfo {
    seeds: Vec<u64>,
    map_infos: Vec<MapInfo>
}
#[derive(Debug, Clone)]
struct MapInfo {
    source_name : String,
    destination_name: String,
    numbers_info: Vec<RangeInfo>
}
#[derive(Debug, Clone)]
struct RangeInfo {
    dest_min:u64,
    dest_max:u64,
    source_min:u64,
    source_max:u64,
    range: u64
}

impl InputInfo {
    pub fn parse_input(input : &str) -> Self {
       let (first, rest) =  input.split_once("\n\n").unwrap();
        let seeds =  str_to_vec_u64(first.split_once(": ").unwrap().1);
        //println!("{:?}", seeds);
        let map_infos = rest.split("\n\n").map(|map_source_dest| MapInfo::parse_map(map_source_dest)).collect::<Vec<MapInfo>>();
        println!("{:?}", rest.lines().next().unwrap());
        //lines
        InputInfo{
            seeds: seeds,
            map_infos
        }
    }
}
impl MapInfo {
    pub fn parse_map(map_source_dest : &str) -> Self {

        let mut lines = map_source_dest.lines();
        let (source, destination) = lines.next().unwrap().split_once(" ").unwrap().0.split_once("-to-").unwrap();

        let numbers_info = lines.map(|range_line| RangeInfo::parse_numbers(range_line)).collect::<Vec<RangeInfo>>();

        MapInfo {
            source_name : source.to_string(),
            destination_name: destination.to_string(),
            numbers_info,
        }
    }
}

impl RangeInfo {
    pub fn parse_numbers(all_numbers: &str) -> Self{
        let nums = str_to_vec_u64(all_numbers);
        let range = nums[2];

        RangeInfo {
            source_min: nums[1],
            source_max:nums[1] + (range -1),
            dest_min: nums[0],
            dest_max: nums[0] + (range -1),
            range
        }
    }
}
fn str_to_vec_u64(numbers : &str) -> Vec<u64> {
    numbers.split(" ").map(|number| number.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}