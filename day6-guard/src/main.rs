use std::{fs, process::Command};

use itertools::Itertools;

fn turn(v: (i32, i32))->(i32, i32){
    let (y,x) = v;
    return (x, -y)
}
fn get<T: Clone>(map: &Vec<Vec<T>>, pos: (usize,usize)) -> T{
    return map[pos.0][pos.1].clone();
}
fn get_i32<T: Clone>(map: &Vec<Vec<T>>, pos: (i32,i32)) -> T{
    let pos = (pos.0 as usize, pos.1 as usize);
    return map[pos.0][pos.1].clone();
}
fn get_mut<T>(map: &mut Vec<Vec<T>>, pos: (usize,usize)) -> &mut T{
    &mut map[pos.0][pos.1]
}
fn get_mut_i32<T>(map: &mut Vec<Vec<T>>, pos: (i32,i32)) -> &mut T{
    let pos = (pos.0 as usize, pos.1 as usize);
    &mut map[pos.0][pos.1]
}

fn get_dir_idx(directions: &Vec<(i32,i32)>, dir: (i32,i32))->usize{
    directions.iter().position(|&x| x==dir).unwrap()
}
fn main() {
    let directions = (0..4).into_iter().map(|x| {let mut v = (-1, 0); for _ in 0..x {v = turn(v)} v} ).collect::<Vec<_>>();
    let input = fs::read_to_string("input").expect("the unexpected");
    // let input = fs::read_to_string("input-test").expect("the unexpected");
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sol = (0,0);
    let size = (map.len(), map[0].len());
    let mut visited: Vec<Vec<[bool; 4]>> = vec![ vec![[false;4]; size.1]; size.0];
    // println!("{map:?}");
    let mut pos = (0,0);
    let mut dir = (-1, 0);
    let mut obstacles = Vec::new();
    for (i, row) in map.iter().enumerate(){
        if let Some(j) = row.iter().position(|&x| x == '^'){
            pos = (i,j);
            break;
        }
    }
    // loop {
    //     let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
    //     if new_pos.0 < 0 || new_pos.1<0 || new_pos.0>=size.0 as i32 || new_pos.1 >= size.1 as i32{
    //         break;
    //     }
    //     let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
    //     let target = get(&map, new_pos);
    //     if target == '#'{
    //         dir = turn(dir);
    //     }
    //     else{
    //         let mut test_dir = turn(dir);
    //         let mut test_pos = pos;
    //         let mut skip = true;
    //         loop {
    //             if test_pos==pos && test_dir==turn(dir) && !skip{
    //                 sol.1+=1;
    //                 obstacles.push((pos.0 as i32 + dir.0, pos.1 as i32 + dir.1));
    //                 break;
    //             }
    //             skip = false;
    //             println!("{test_pos:?}");
    //             // if get(&visited, test_pos)[get_dir_idx(&directions, test_dir)]{
    //             //     sol.1 += 1;
    //             //     break;
    //             // };
    //             let new_test_pos = (test_pos.0 as i32 + test_dir.0, test_pos.1 as i32 + test_dir.1);
    //             if new_test_pos.0 < 0 || new_test_pos.1<0 || new_test_pos.0>=size.0 as i32 || new_test_pos.1 >= size.1 as i32{
    //                 break;
    //             }
    //             test_pos = (new_test_pos.0 as usize, new_test_pos.1 as usize);
    //             if get(&map, test_pos)=='#' {
    //                 test_dir = turn(test_dir);
    //             };

    //         }

    //         if !get(&visited, pos).iter().all(|&x| x){
    //             sol.0+=1;
    //             get_mut(&mut visited, pos)[get_dir_idx(&directions, dir)] = true; 
    //             // set(&mut visited, pos, true);
    //         }
    //         pos = new_pos;
    //     }
    // }

    let mut warp = None;
    let mut warp_visited: Vec<Vec<[bool; 4]>> = vec![ vec![[false;4]; size.1]; size.0];
    let mut warp_ready = false;
    // for i in 0..20 {println!()};
    // for row in  map[36..map.len()-40].iter() {
    //     println!("{}", row.iter().collect::<String>())
    // }
    loop {
        // for row in  map[0..map.len()-76].iter() {
        for row in  map[36..map.len()-40].iter() {

        // println!("");
        // for row in  map.iter() {
            println!("{}", row.iter().collect::<String>())
        }
        Command::new("sleep").arg("0.01").spawn().unwrap().wait().unwrap();
        // Command::new("sleep").arg("0.1").spawn().unwrap().wait().unwrap();

        let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if new_pos.0 < 0 || new_pos.1<0 || new_pos.0>=size.0 as i32 || new_pos.1 >= size.1 as i32{
            if let Some((p,d,_t, target)) = warp{
                pos=p;
                dir=d;
                warp=None;
                // *get_mut_i32(&mut map, (p.0 as i32 + d.0, p.1 as i32 + d.1)) = '.';
                *get_mut(&mut map, target) = '.';
                continue;
            }
            break;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        let target = get(&map, new_pos);

        if target == '#'{
            dir = turn(dir);
        }
        else{
            if (warp == None && !warp_ready) || warp.is_some_and(|(p,d,t, _)| t>0) {*get_mut(&mut map, pos) = '.'};
            *get_mut(&mut map, new_pos) = if warp==None {'^'} else {'o'};
            match warp {
                Some((p,d,t, target)) => {
                    // if (p,d)==(pos,dir){
                    //         obstacles.push((p.0 as i32 + d.0, p.1 as i32 + d.1));
                    //         sol.1+=1;
                    //         warp = None;
                    //         pos = p;
                    //         dir = d;
                    //         *get_mut_i32(&mut map, (p.0 as i32 + d.0, p.1 as i32 + d.1)) = '.';
                    //         continue;
                    // }
                    // else
                    if get(&warp_visited, pos)[get_dir_idx(&directions, dir)] {
                        obstacles.push((p.0 as i32 + d.0, p.1 as i32 + d.1));
                        sol.1+=1;
                        warp = None;
                        // *get_mut(&mut map, pos) = '-';
                        // *get_mut_i32(&mut map, (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1)) = '+';
                        pos = p;
                        dir = d;
                        // *get_mut_i32(&mut map, (p.0 as i32 + d.0, p.1 as i32 + d.1)) = '.';
                        *get_mut(&mut map, target) = '.';
                        continue;
                    }
                    // else
                        
                    // else 
                    // if t>10000{
                    //     obstacles.push((p.0 as i32 + d.0, p.1 as i32 + d.1));
                    //     sol.1+=1;
                    //     warp = None;
                    //     pos = p;
                    //     dir = d;
                    //     *get_mut_i32(&mut map, (p.0 as i32 + d.0, p.1 as i32 + d.1)) = '.';
                    //     continue;
                    // }

                    get_mut(&mut warp_visited, pos)[get_dir_idx(&directions, dir)] = true;
                    *get_mut(&mut map, pos) = '-';
                    if warp != None {warp = Some((p,d,t+1,target))};
                    pos = new_pos;
                },
                None => {
                    if !get(&visited, pos).iter().any(|&x| x){
                        sol.0+=1;
                        // set(&mut visited, pos, true);
                    }
                    // let t_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                    // if !(t_pos.0 < 0 || t_pos.1<0 || t_pos.0>=size.0 as i32 || t_pos.1 >= size.1 as i32){
                    //     if get_i32(&map, t_pos) != '#'{
                        //         warp = Some((pos, dir, 0));
                        //         warp_visited = visited.clone();
                        //         // *get_mut_i32(&mut map, t_pos) = '#';
                        //         // dir = turn(dir);
                        //     }
                        // }
                    get_mut(&mut visited, pos)[get_dir_idx(&directions, dir)] = true; 
                    // *get_mut(&mut map, pos) = '+';
                    if warp_ready && !get(&visited, new_pos).iter().any(|&x|x) {
                        warp = Some((pos, dir, 0, new_pos));
                        for (i, row) in warp_visited.iter().enumerate(){
                            for (j, &p) in row.iter().enumerate(){
                                if p.iter().any(|x| *x) { *get_mut(&mut map, (i,j)) = '.'; }
                            }
                        }
                        for (i, row) in visited.iter().enumerate(){
                            for (j, &p) in row.iter().enumerate(){
                                if p.iter().any(|x| *x) { *get_mut(&mut map, (i,j)) = '+'; }
                            }
                        }
                        warp_visited = visited.clone();
                        
                        // warp_visited = vec![ vec![[false;4]; size.1]; size.0];
                        *get_mut(&mut map, new_pos) = '#';
                        warp_ready = false
                    }
                    else{
                        pos = new_pos;
                        warp_ready = true;
                    }
                },
            }
        }
    }

    sol.0 +=1;
    println!("{sol:?}");
    println!("{:?}", obstacles.iter().unique().count());
    println!("{:?}", obstacles.iter().unique().filter(|&&p| get_i32(&map, p)!='#').count());
    println!("{:?}", directions);
    println!("{:?}", directions.iter().map(|&d| get_dir_idx(&directions, d)).collect::<Vec<_>>());
    println!("{:?}", turn(directions[3]));
}
