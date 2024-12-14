// use advent_of_code_question::*;

use aoc::tuples_galore::{add2, map2, operate2, sub2};
use itertools::Itertools;

fn main() {
    let mut sol = (1,1);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    // let size = (11,7);
    let size = (101,103);
    let warp = |pos: (i32,i32)| { operate2(pos, size, |p, bound| { p.rem_euclid(bound)}) };
    let mut robots = vec![];
    for line in input.lines(){
        let (p,v) = line.split_once(" v=").unwrap();
        let p: (i32,i32) = p[2..].split(",").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let v: (i32,i32) = v.split(",").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        robots.push(
            (p,v)
        );
    }
    for t in 0..101*103{
        // let mut neighbourships = 0;
        // for (i ,(p0,_)) in robots.iter().enumerate(){
        //     for (p1,_) in robots[i+1..].iter(){
        //         let dp =  sub2(*p0, *p1);
        //         let dp = map2(dp, |x| i32::abs(x));
        //         if dp.0+dp.1 == 1 {neighbourships += 1}
        //     }
        // }
        // print!("{}",['█';101].into_iter().collect::<String>());
        // if neighbourships > 70 {
        //     println!(" elapsed seconds: {}\t\tn = {neighbourships}", t );
        //     show(&robots, Some(size))
        // }
        if has_line(&robots, size){
            println!(" elapsed seconds: {}\t\t", t );
            show(&robots, Some(size))
        }
        // show(&robots, Some(size));
        for (p, v) in robots.iter_mut(){
            *p = warp(add2(*p, *v));
        }
    }
    print!("{}",['█';101].into_iter().collect::<String>());
    println!(" elapsed seconds: {}", 100 );
    show(&robots, Some(size));

    let mut quads = [[0;2];2];
    for (p,_) in robots.iter(){
        let bounds = map2(size, |x| (x)/2);
        if p.0 != bounds.0 && p.1 != bounds.1{
            let (i,j) = operate2(*p, bounds, |p, bound| { 
                if p <bound {0}
                else {1}
            } );
            quads[j as usize][i as usize] += 1
        }
    }
    for row in quads{
        println!("{row:?}");
        for quad in row{
            sol.0 *= quad;
        }
    }
    println!("\n solutions: {sol:?}");
}

fn show(robots: &Vec<((i32, i32), (i32, i32))>, map_size: Option<(i32,i32)>) {
    if let Some(size) = map_size{
        let mut map = vec![vec!['.';size.0 as usize];size.1 as usize];
        for (p,_) in robots {
            map[p.1 as usize][p.0 as usize] = '█';
        }
        for row in map{
            for c in row{
                print!("{c}");
            }
            println!();
        }
    }
    else {
        for robot in robots{
            println!("{robot:?}")
        }
        println!("");
    }
}
fn has_line(robots: &Vec<((i32, i32), (i32, i32))>, size: (i32,i32)) -> bool {
    let mut map = vec![vec![false;size.0 as usize];size.1 as usize];
    for (p,_) in robots {
        map[p.1 as usize][p.0 as usize] = true;
    }
    for row in map{
        if row.iter().filter(|x| **x).count() > 30 {return true};
    }
    false
}