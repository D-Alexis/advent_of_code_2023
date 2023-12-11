use std::time::Instant;

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    println!("parsing took : {:?}", start.elapsed());

    let start = Instant::now();
      part1(input);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2(input);
    println!("day2 took : {:?}", start.elapsed());
}
fn part1(input: &str) {
    //parsing
    let mut empty_lines:Vec<usize> = Vec::with_capacity(10);
    let mut empty_col:Vec<usize> = Vec::with_capacity(10);
    let mut space : Vec<Vec<char>> = Vec::with_capacity(150); //(140+10);
    input.lines().for_each(|line| {
        let mut is_empty = true;
        let new_line = line.chars().map(|cur_char|{
            if cur_char == '#' {
                is_empty = false;
            }
            cur_char
        }).collect::<Vec<char>>();
        if is_empty {
            empty_lines.push(space.len());
        }
        space.push(new_line);
    });

    for j in 0..space.len() {
        let mut is_empty = true;
        for i in 0..space.len() {
            if space[i][j] == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_col.push(j)
        }
    }
    empty_lines.reverse();
    empty_lines.iter().for_each(|index| {
        space.insert(*index, vec!['.';140]);
    });
    empty_col.reverse();
    empty_col.iter().for_each(|index| {
        for i in 0..space.len() {
            space[i].insert(*index, '.');
        }
    });

    let mut all_galax: Vec<(isize,isize)> = Vec::with_capacity(300);
    for i in 0..space.len() {
        for j in 0..space[0].len() {
            if space[i][j] == '#' {
                all_galax.push((i as isize, j as isize));
            }
        }
    }

    // parsing done

    let mut all_distance : Vec<usize> = Vec::with_capacity((all_galax.len() *(all_galax.len() -1))/2 ); // (n * (n+1)) /2 the round robin rule
    for i in 0..all_galax.len()-1  {
        for j in i+1..all_galax.len() {
            all_distance.push(dist_man(all_galax[i], all_galax[j]))
        }
    }
    println!("{:?}", all_distance.iter().sum::<usize>());
}

fn dist_man(point_a: (isize,isize), point_b: (isize,isize)) -> usize {
    point_b.1.abs_diff(point_a.1) +  point_b.0.abs_diff(point_a.0)
}
fn part2(input: &str) {
    let factor = 1_000_000;
    //parsing
    let mut empty_lines:Vec<usize> = Vec::with_capacity(10);
    let mut empty_col:Vec<usize> = Vec::with_capacity(10);
    let mut space : Vec<Vec<char>> = Vec::with_capacity(150); //(140+10);
    input.lines().for_each(|line| {
        let mut is_empty = true;
        let mut new_line: Vec<char> = Vec::with_capacity(150);
        line.chars().for_each(|cur_char|{
            if cur_char == '#' {
                is_empty = false;
            }
            new_line.push(cur_char);
        });
        if is_empty {
            empty_lines.push(space.len());
        }
        space.push(new_line);
    });

    for j in 0..space.len() {
        let mut is_empty = true;
        for i in 0..space.len() {
            if space[i][j] == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_col.push(j)
        }
    }

    let mut all_galax: Vec<(isize,isize)> = Vec::with_capacity(300);
    let mut line_factor = 0;

    for i in 0..space.len() {
        if empty_lines.contains(&i) {
            line_factor = line_factor + (factor-1)
        }
        let mut col_factor = 0;
        for j in 0..space[0].len() {
            if empty_col.contains(&j) {
                col_factor = col_factor + (factor-1)
            }
            if space[i][j] == '#' {
                all_galax.push(((i + line_factor) as isize, (j+ col_factor) as isize));
            }
        }
    }
    // parsing done

    let mut all_distance : Vec<usize> = Vec::with_capacity((all_galax.len() *(all_galax.len() -1))/2 ); // (n * (n+1)) /2 the round robin rule
    for i in 0..all_galax.len()-1  {
        for j in i+1..all_galax.len() {
            all_distance.push(dist_man(all_galax[i], all_galax[j]))
        }
    }
    println!("{:?}", all_distance.len());
    println!("{:?}", all_distance.iter().sum::<usize>());
}