// use advent_of_code_question::*;

use aoc::{array2d::Array2D, input::{as_array2d, print2d, size2d}, tuples_galore::{add2, map2}};
use itertools::Itertools;

fn turn_c(dir: (i128,i128)) -> (i128,i128) { (dir.1, -dir.0) }
fn turn_cc(dir: (i128,i128)) -> (i128,i128) { (-dir.1, dir.0) }

fn dir_id(dir: (i128,i128)) -> usize {
    match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => 4
    }
}

fn main() {
    let mut sol = (None,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    let mut map = as_array2d(&input);
    let size = map2(size2d(&map), |x| x as i128);
    let start_pos = (size.0 - 2, 1);
    let end_pos = (1, size.1-2);
    let start_dir = (0,1);

    let mut visited: Vec<Vec<[Option<i128>; 4]>> = vec![vec![[None;4]; size.1 as usize]; size.0 as usize];

    let mut paths = vec![(start_pos, start_dir, 0, vec![start_pos])];
    let mut best = vec![];
    while let Some((pos, dir, score, path)) = paths.pop() {
        let did = dir_id(dir);
        if let Some(min_local_score) = visited.try_get2d(pos).unwrap()[did]{
            if score > min_local_score{
                continue;
            }
        }
        visited.try_get2d_mut(pos).unwrap()[did] = Some(score);

        if pos==end_pos{
            if let Some(min_score) = sol.0 {
                if score > min_score{
                    continue;
                }
            }
            sol.0 = Some(score);
            best.push((path,score));
            continue;
        }

        let new_pos = add2(pos, dir);
        let new_dir_c = turn_c(dir);
        let new_pos_c = add2(pos, new_dir_c);
        let new_dir_cc = turn_cc(dir);
        let new_pos_cc = add2(pos, new_dir_cc);

        if let Some(&c) = map.try_get2d(new_pos) {
            if c != '#'{
                let mut new_path = path.clone();
                new_path.push(new_pos);
                // paths.push((new_pos, dir, score+1, new_path));
                paths.insert(0,(new_pos, dir, score+1, new_path));  
                // breadth first (insert[0] instead of push) is better, cuz it eliminates longer overlaping paths at their roots
                // while doing depth first you could reach the end and search all around it 
                // before realizing that the path you're checking never be actually the best path
                // because in the first few turns there's a path that can reach same position (and direction) with better score
            }
        }
        if let Some(&c) = map.try_get2d(new_pos_c) {
            if c != '#'{
                let mut new_path = path.clone();
                new_path.push(new_pos_c);
                // paths.push((new_pos_c, new_dir_c, score+1001, new_path));
                paths.insert(0,(new_pos_c, new_dir_c, score+1001, new_path));
            }
        }
        if let Some(&c) = map.try_get2d(new_pos_cc) {
            if c != '#'{
                let mut new_path = path.clone();
                new_path.push(new_pos_cc);
                // paths.push((new_pos_cc, new_dir_cc, score+1001, new_path));
                paths.insert(0,(new_pos_cc, new_dir_cc, score+1001, new_path));
            }
        }
    }
    let mut sol = (sol.0.unwrap(), sol.1);
    let mut sits = vec![];
    for (path, score) in best{
        if score==sol.0{
            for p in path{
                if sits.iter().find(|s| **s==p).is_none(){
                    sits.push(p);
                }
            }
        }
    }
    sol.1 = sits.iter().unique().count();


    print2d(&map);

    println!("\n solutions: {sol:?}");
}
