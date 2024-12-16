// use advent_of_code_question::*;

use aoc::{array2d::{enumerate_array2d, Array2D}, input::as_array2d, tuples_galore::{add2, map2, mul2, sub2}};


fn main() {
    let dir: [(char, (i32, i32)); 4] = [
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('<', (0, -1)),
        ('^', (-1, 0)),
    ];
    let as_dir = |c: char| -> (i32, i32) {dir.iter().find(|(cd,_)| *cd==c).unwrap().1};
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut map = as_array2d(map);
    let mut pos = map.iter().enumerate().fold(None,|acc, (i,row)| {
        acc.or(
            row.iter().position(|c| *c=='@').map(|j| (i as i32,j as i32))
        )
    }).unwrap();
    let mut map_p2 = vec![];
    for row in &map{
        let mut row_p2 = vec![];
        for &c in row{
            let sub = match c {
                '@' => "@.",
                'O' => "[]",
                c => &format!("{c}{c}"),
            };
            for c in sub.chars() { row_p2.push(c); }
        }
        map_p2.push(row_p2);
    }

    for line in instructions.lines(){ for instruction in line.chars(){
        // print2d(&map);

        let dir = as_dir(instruction);
        let mut check_pos = pos;
        let mut target = None;
        let positioned_map = enumerate_array2d(&map);
        while let Some((p, &c)) = positioned_map.try_get2d(check_pos) {
            if c == '#'{
                break;
            }
            if c == '.' {
                target = Some(map2(*p, |x| x as i32));
                break;
            }
            check_pos = add2(check_pos, dir);
        }

        // println!("{pos:?} {instruction}");
        // println!("target: {target:?}");

        if let Some(mut p) = target {
            let mut np = sub2(p, dir);
            while p != pos  {
                let c = *map.try_get2d(np).unwrap();
                *map.try_get2d_mut(p).unwrap() = c;
                // println!("\t [{c}] {np:?}->{p:?}");
                p = np;
                np = sub2(np, dir)
            }
            *map.try_get2d_mut(pos).unwrap() = '.';
            pos = add2(pos, dir);
        }
    }}
    // print2d(&map);
    for row in enumerate_array2d(&map){ for ((i,j), &c) in row{
        if c == 'O'{
            sol.0 += 100*i + j;
        }
    }}
    
    let mut pos = map_p2.iter().enumerate().fold(None,|acc, (i,row)| {
        acc.or(
            row.iter().position(|c| *c=='@').map(|j| (i as i32,j as i32))
        )
    }).unwrap();
    for line in instructions.lines(){ for instruction in line.chars(){
        // print2d(&map_p2);

        let dir = as_dir(instruction);
        let mut to_check: Vec<(i32, i32)> = vec![pos];
        let mut to_move = vec![];
        let mut to_clear = vec![pos];
        let mut not_to_clear = vec![];
        // let mut target = None;

        println!("{pos:?} {instruction}");
        while let Some(&p) = to_check.last() {
            // println!("target: {target:?}");
            if let Some( &c) = map_p2.try_get2d(p){
                println!("\tchecking: {c}");
                if c == '#'{
                    to_move.clear();
                    break;
                }
                if c == '.'{
                    to_check.pop();
                }
                else {
                    to_move.push(p);
                    to_check.pop();
                    to_check.push(add2(p, dir));
                    if dir.0 != 0{
                        if c=='['{
                            let box_p = add2(p, (0,1));
                            to_clear.push(box_p);
                            not_to_clear.push(p);
                            to_move.push(box_p);
                            to_check.push(add2(box_p, dir));
                        }
                        if c==']'{
                            let box_p = add2(p, (0,-1));
                            to_clear.push(box_p);
                            not_to_clear.push(p);
                            to_move.push(box_p);
                            to_check.push(add2(box_p, dir));
                        }
                    }
                }
            }
            else { 
                to_move.clear();
                break;
            }
        }
        // to_clear = vec![pos];
        // for p in to_move{
        //     if let Some(p1) = to_clear.iter().find(|&&x| mul2(x, dir) == mul2(x, y))
        // }
        to_move.sort_by_key(|p| {
            let dp = mul2(sub2(*p, pos), dir);
            -(dp.0+dp.1)
        });
        println!("\tmoving {to_move:?}");
        for &p in &to_move  {
            let np = add2(p, dir);
            let c = *map_p2.try_get2d(p).unwrap();
            *map_p2.try_get2d_mut(np).unwrap() = c;
            println!("\t [{c}] {np:?}->{p:?}");
        }
        if !to_move.is_empty(){
            for p in to_clear{
                if not_to_clear.iter().find(|x| **x==p).is_none(){
                    *map_p2.try_get2d_mut(p).unwrap() = '.';
                }
            }
            pos = add2(pos, dir);
        }
    }}
    print2d(&map_p2);
    // for row in enumerate_array2d(&map_p2){ for ((i,j), &c) in row{
    //     if c == 'O'{
    //         sol.1 += 100*i + j;
    //     }
    // }}
    // print2d(&map);
    for row in enumerate_array2d(&map_p2){ for ((i,j), &c) in row{
        if c == '['{
            sol.1 += 100*i + j;
        }
    }}


    println!("{pos:?}");

    println!("\n solutions: {sol:?}");
}

fn print2d(map: &Vec<Vec<char>>) {
    for row in map{
        for c in row{
            print!("{c}");
        }
        println!();
    }
}
