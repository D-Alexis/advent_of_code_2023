use std::time::Instant;

fn main() {
    let input = include_str!("input_test.txt");
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
    let (seed_start, seed_end) : (u64, u64) = (79, 92);
    let init = RangeInfo {
        dest_min: seed_start,
        dest_max: seed_end,
        source_min: seed_start,
        source_max: seed_end,
        range: seed_end-seed_start
    };


    let mut iter= input_info.map_infos.iter();
    let mut result_end_map = iter.next().unwrap();
    result_end_map.numbers_info.iter().for_each(|number_info| {
       let new_vec = compute_intersection(&init,number_info);

       println!("{:?} and {:?}",number_info, new_vec);
    });


  //  get_that_fucking_map(&input_info.map_infos);
}


fn compute_intersection(seed_range_info : &RangeInfo, next_step_range_info: &RangeInfo) -> Vec<RangeInfo> {
    let mut seed_min = seed_range_info.dest_min;
    let mut seed_max = seed_range_info.dest_max;
    let mut cur_min = next_step_range_info.source_min;
    let mut cur_max = next_step_range_info.source_max;
    let mut res : Vec<RangeInfo> = Vec::with_capacity(100);
    let  (mut inter_min, mut inter_max) = (u64::MAX, u64::MAX);

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
            res.push(RangeInfo {
                dest_min: inter_min + (next_step_range_info.dest_min - next_step_range_info.source_min),
                dest_max: inter_max + (next_step_range_info.dest_min - next_step_range_info.source_min),
                source_min: seed_range_info.source_min,
                source_max: seed_range_info.source_max,
                range: 0,
            })
        } else {
            // case :
            //  -----
            // -----
            //
            inter_max = cur_max;
            let (dest_min, dest_max) = get_min_max(next_step_range_info, inter_min, inter_max);
            res.push(RangeInfo {
                dest_min,
                dest_max,
                source_min: seed_range_info.source_min,
                source_max: inter_max,
                range: 0,
            });
            res.push(RangeInfo {
                dest_min: inter_max+1,
                dest_max: seed_range_info.dest_max,
                source_min: inter_max+1,
                source_max: seed_range_info.source_max,
                range: 0,
            })
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
            let (dest_min, dest_max) = get_min_max(next_step_range_info, inter_min, inter_max);
            res.push(RangeInfo {
                dest_min,
                dest_max,
                source_min: inter_min,
                source_max: seed_range_info.source_max,
                range: 0,
            });
            res.push(RangeInfo {
                dest_min: seed_range_info.dest_min,
                dest_max: inter_min-1,
                source_min: seed_range_info.source_min,
                source_max: inter_min-1,
                range: 0,
            })
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
        res.push(RangeInfo {
            dest_min: seed_range_info.dest_min,
            dest_max: inter_min-1,
            source_min: seed_range_info.source_min,
            source_max: inter_min-1,
            range: 0,
        });
        let (dest_min, dest_max) = get_min_max(next_step_range_info, inter_min, inter_max);
        res.push(RangeInfo {
            dest_min,
            dest_max,
            source_min: inter_min,
            source_max: inter_max,
            range: 0,
        });
        res.push(RangeInfo {
            dest_min: inter_max+1,
            dest_max: seed_range_info.dest_max,
            source_min: inter_max+1,
            source_max: seed_range_info.source_max,
            range: 0,
        });
    }
    res
}

fn get_min_max(next_step_range_info: &RangeInfo, inter_min:u64, inter_max:u64) -> (u64, u64) {
    let (dest_min, dest_max) = if next_step_range_info.dest_min > next_step_range_info.source_min {
        (inter_min + (next_step_range_info.dest_min - next_step_range_info.source_min),
         inter_max + (next_step_range_info.dest_min - next_step_range_info.source_min))
    } else {
        (inter_min - (next_step_range_info.source_min - next_step_range_info.dest_min),
         inter_max - (next_step_range_info.source_min - next_step_range_info.dest_min))
    };
    (dest_min, dest_max)
}
/*
fn get_that_fucking_map(map_infos : &Vec<MapInfo>) {
    let mut iter= map_infos.iter();
    let mut result_end_map = iter.next().unwrap().numbers_info.clone();

    // pour toutes les autres mapinfo, on va calculer l'ensemble des range toutDebut -> touteFin
    iter.for_each(|map_info|  {

        map_info.numbers_info.iter().for_each(|range_info| {
            //on met à jour chaque range, et potentiellement on en écrit d'autres.
            compute_info(&mut result_end_map, range_info);
        });
    });
}
*/
/*
fn compute_info(result_end_map: &mut Vec<RangeInfo>, range_info: &RangeInfo) {
    let mut new_range_info: Vec<RangeInfo> = Vec::with_capacity(5);
    result_end_map.iter_mut().for_each(|current_result_range| {
        println!("current_res : {:?} ",current_result_range);
        println!("range_info : {:?} ",range_info);
        //update range
        if let Some((inter_min, inter_max)) =  get_range_intersection((current_result_range.dest_min, current_result_range.dest_max),
                                                                    (range_info.source_min, range_info.source_max)) {
            println!("{inter_min} and {inter_max}");
        } else {
            println!("None");
        }
    });
}
*/

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
       let (first, rest) =  input.split_once("\r\n\r\n").unwrap();
        let seeds =  str_to_vec_u64(first.split_once(": ").unwrap().1);
        //println!("{:?}", seeds);
        let map_infos = rest.split("\r\n\r\n").map(|map_source_dest| MapInfo::parse_map(map_source_dest)).collect::<Vec<MapInfo>>();
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