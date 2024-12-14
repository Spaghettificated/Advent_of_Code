// use advent_of_code_question::*;

use advent_of_code_question::{array2d::{enumerate_array2d, Array2D}, input::as_mapped_array2d, tuples_galore::{add2, map2, sub2}};

fn main() {
    let dirs = [(0,1),(1,0),(0,-1),(-1,0)];
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    let mut map = as_mapped_array2d(&input, |c| Some(c));
    // let size = (map.len(), map[0].len());
    let mut to_check = vec![(vec![(0,0)], (0,0, map.get2d((0,0)).unwrap().unwrap()) )];

    // println!("");
    // for line in map.iter(){
    //     println!("{line:?}")
    // }

    // let mut starting_pos;
    'main: loop {
        println!("");
        for line in map.iter(){
            println!("{}", line.iter().map(|x| x.unwrap_or('.')).collect::<String>())
        }
        let mut to_check = vec![];
        let (mut area, mut perimeter) = (0,0);
        let mut c = '?';
        let mut sides = vec![];
        let mut tile_positions = vec![];
        // let mut found = false;
        'find_starting_pos: for row in enumerate_array2d(&map) {
            for (pos, x) in row{
                if x.is_some() {
                    to_check.push(map2(pos, |x| x as i32));
                    c = x.unwrap();
                    // found = true;
                    break 'find_starting_pos;
                }
            }
        }
        // if !found {break 'main;}
        if to_check.is_empty() {break 'main;}
        while let Some(pos) = to_check.pop() {
            if let Some(tile) = map.try_get2d(pos){
                tile_positions.push(pos);
                let tile = *tile;
                println!("\tpoping: {pos:?} with {tile:?}");
                if let Some(c) = tile {
                    area += 1;
                    perimeter += 4;
                    for dir in dirs{
                        let n_pos = add2(pos, dir);
                        if let Some(Some(neighbour_c)) = map.try_get2d(n_pos){
                            if *neighbour_c==c{
                                to_check.push(n_pos);
                                perimeter -= 2;
                                // to_check.last_mut().unwrap().1.1 -= 1;
                                // to_add.push(add2(pos, dir));
                                // to_check.last_mut().unwrap().0.push(add2(pos, dir));
                            }
                            else {
                                sides.push((pos, dir));
                            }
                        }
                        else {
                            sides.push((pos, dir));
                        }
                    }
                }
                
                *map.try_get2d_mut(pos).unwrap() = None;
            }
        }
        println!("\tadding {area}*{perimeter} for {c:?}");
        sol.0 += area*perimeter;
        
        println!("\tsides {sides:?}");
        let mut side_len = 0;
        // for side in sides.clone(){

        // }
        //879165
        for (i,&side) in sides.clone().iter().enumerate().rev(){
            if let Some(_) = tile_positions.iter().position(|pos1|{
                *pos1 == add2(side.0, side.1) 
            }) {
                sides.remove(i);
            }
        }
        while let Some((pos, dir)) = sides.pop() {
            if let Some(i) = tile_positions.iter().position(|pos1|{
                *pos1 == add2(pos, dir) 
            }) {
                continue;
            }
            while let Some(i) = sides.iter().position(|(pos1,dir1)|{
                *pos1 == pos && *dir1 == dir
            }) {
                sides.remove(i);
            }

            let mut side_parts = vec![pos];
            while let Some(i) = sides.iter().position(|(pos1, dir1)|{
                if *dir1==dir 
                {
                    for &pos0 in &side_parts{
                        if add2(pos0, (dir.1, dir.0) ) == *pos1 {return true;}
                        if sub2(pos0, (dir.1, dir.0) ) == *pos1 {return true;}
                    }
                }
                false
            }) {
                side_parts.push(sides[i].0);
                sides.remove(i);
            }
            println!("\t\t side facing {dir:?}: {side_parts:?}");
            side_len += 1;
        }
        println!("\tadding part2 {area}*{side_len} for {c:?}");
        sol.1 += area*side_len;
        // sol.1 += 
        // for (i, (pos, dir)) in sides.iter().enumerate(){

        // }
    }


    println!("\n solutions: {sol:?}");
}
