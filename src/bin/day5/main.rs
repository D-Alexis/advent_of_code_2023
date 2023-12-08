use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    let input_info = InputInfo::parse_input(input);
    println!("parsing took : {:?}", start.elapsed());

    /*
       let start = Instant::now();
        day1(&input_info);
        println!("day1 took : {:?}", start.elapsed());
    */

    let start = Instant::now();
    day2(&input_info);
    println!("day2 took : {:?}", start.elapsed());
}

fn day1(input_info: &InputInfo) {
    let mut source = input_info.seeds.clone();

    input_info
        .map_infos
        .iter()
        .for_each(|map_info| source = find_all_next(&source, map_info));
    println!("{:?}", source.iter().min().unwrap());
}

fn find_all_next(sources: &Vec<i64>, map_info: &MapInfo) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::with_capacity(sources.len());
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

fn day2(input_info: &InputInfo) {
    let all_seed_range = &input_info.seed_range;

    let mut iter = input_info.map_infos.iter();
    let mut current_map = iter.next().unwrap();

    let min = all_seed_range
        .iter()
        .map(|init| {
            let mut current: HashSet<(i64, i64)> = HashSet::with_capacity(1000);
            let mut next: HashSet<(i64, i64)> = HashSet::with_capacity(1000);
            current.insert(*init);
            input_info.map_infos.iter().for_each(|current_map| {
                // START parcours des mapping
                current_map.numbers_info.iter().for_each(|number_info| {
                    // parcours des ligne du mapping
                    current.iter().for_each(|current_seed_range| {
                        compute_intersection(*current_seed_range, number_info, &mut next);
                    });
                });
                // END parcours d'un mapping
                if next.len() != 0 {
                    current = next.clone();
                    next.clear();
                }

                let minn = *current.iter().map(|(min, max)| min).min().unwrap();
            });
            let minn = *current.iter().map(|(min, max)| min).min().unwrap();
            println!("{:?}", minn);
            minn
        })
        .min()
        .unwrap();
    println!("{:?}", min);
    //  get_that_fucking_map(&input_info.map_infos);
}

fn compute_intersection(
    seed_range_info: (i64, i64),
    next_step_range_info: &RangeInfo,
    new_vec: &mut HashSet<(i64, i64)>,
) {
    let mut seed_min = seed_range_info.0;
    let mut seed_max = seed_range_info.1;
    let mut cur_min = next_step_range_info.source_min;
    let mut cur_max = next_step_range_info.source_max;
    let (mut inter_min, mut inter_max) = (i64::MAX, i64::MAX);
    let diff = next_step_range_info.dest_min - next_step_range_info.source_min;

    if seed_min >= cur_min && seed_min <= cur_max {
        // case :
        //  ---
        // ---
        //
        inter_min = seed_min;
        if seed_max <= cur_max {
            // case :
            //  ---
            // -----
            //
            inter_max = seed_max;

            new_vec.insert((inter_min + diff, inter_max + diff));
        } else {
            // case :
            //  -----
            // -----
            //
            inter_max = cur_max;
            new_vec.insert((inter_min + diff, inter_max + diff));
            new_vec.insert((inter_max + 1, seed_max));
        }
    } else if seed_max >= cur_min && seed_max <= cur_max {
        // case :
        // ---
        //  ---
        //
        inter_max = seed_max;
        if cur_min >= seed_min {
            // case :
            //   ---
            //    ---
            //
            inter_min = cur_min;
            new_vec.insert((inter_min + diff, inter_max + diff));
            new_vec.insert((seed_min, inter_min - 1));
        } else {
            // case :
            //    ---
            //   -----
            //
            panic!("should have been already treated");
        }
    } else if cur_min >= seed_min && cur_min <= seed_max {
        // case :
        //  ----
        //   --
        //
        inter_max = cur_max;
        inter_min = cur_min;
        new_vec.insert((seed_min, inter_min - 1));
        new_vec.insert((inter_min + diff, inter_max + diff));
        new_vec.insert((inter_max + 1, seed_max));
    }
}

fn get_min_max(next_step_range_info: &RangeInfo, inter_min: i64, inter_max: i64) -> (i64, i64) {
    let (dest_min, dest_max) = if next_step_range_info.dest_min > next_step_range_info.source_min {
        (
            inter_min + (next_step_range_info.dest_min - next_step_range_info.source_min),
            inter_max + (next_step_range_info.dest_min - next_step_range_info.source_min),
        )
    } else {
        (
            inter_min - (next_step_range_info.source_min - next_step_range_info.dest_min),
            inter_max - (next_step_range_info.source_min - next_step_range_info.dest_min),
        )
    };
    (dest_min, dest_max)
}

#[derive(Debug)]
struct InputInfo {
    seeds: Vec<i64>,
    seed_range: Vec<(i64, i64)>,
    map_infos: Vec<MapInfo>,
}
#[derive(Debug, Clone)]
struct MapInfo {
    source_name: String,
    destination_name: String,
    numbers_info: Vec<RangeInfo>,
}
#[derive(Debug, Clone)]
struct RangeInfo {
    dest_min: i64,
    dest_max: i64,
    source_min: i64,
    source_max: i64,
    range: i64,
}

impl InputInfo {
    pub fn parse_input(input: &str) -> Self {
        let (first, rest) = input.split_once("\n\n").unwrap();
        let mut seeds = Vec::with_capacity(20); //str_to_vec_i64(first.split_once(": ").unwrap().1);
        let mut seed_range = Vec::with_capacity(20);
        let all = first.split_once(": ").unwrap().1;
        all.split(",").for_each(|range| {
            let (min, range) = range.split_once(":").unwrap();
            let min = min.parse::<i64>().unwrap();
            let range = range.parse::<i64>().unwrap();
            let max = min + range;
            seeds.push(min);
            seeds.push(range);
            seed_range.push((min, max));
        });
        //println!("{:?}", seeds);
        let map_infos = rest
            .split("\n\n")
            .map(|map_source_dest| MapInfo::parse_map(map_source_dest))
            .collect::<Vec<MapInfo>>();
        println!("{:?}", rest.lines().next().unwrap());
        //lines
        InputInfo {
            seeds,
            map_infos,
            seed_range,
        }
    }
}
impl MapInfo {
    pub fn parse_map(map_source_dest: &str) -> Self {
        let mut lines = map_source_dest.lines();
        let (source, destination) = lines
            .next()
            .unwrap()
            .split_once(" ")
            .unwrap()
            .0
            .split_once("-to-")
            .unwrap();

        let numbers_info = lines
            .map(|range_line| RangeInfo::parse_numbers(range_line))
            .collect::<Vec<RangeInfo>>();

        MapInfo {
            source_name: source.to_string(),
            destination_name: destination.to_string(),
            numbers_info,
        }
    }
}

impl RangeInfo {
    pub fn parse_numbers(all_numbers: &str) -> Self {
        let nums = str_to_vec_i64(all_numbers);
        let range = nums[2];

        RangeInfo {
            source_min: nums[1],
            source_max: nums[1] + (range - 1),
            dest_min: nums[0],
            dest_max: nums[0] + (range - 1),
            range,
        }
    }
}
fn str_to_vec_i64(numbers: &str) -> Vec<i64> {
    numbers
        .split(" ")
        .map(|number| number.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
