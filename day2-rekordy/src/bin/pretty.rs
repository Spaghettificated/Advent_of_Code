// use std::env;
use std::{cmp::Ordering, fs, iter};

fn compare(a: i32, b: i32) -> Ordering{
    match b-a {
        d if d>0 && d<=3 => {Ordering::Greater}
        d if d<0 && d>=-3 => {Ordering::Less}
        _ => {Ordering::Equal}
    }
}

fn main() {
    let data = fs::read_to_string("input").expect("what did you expect?");
    let mut solutions = (0, 0);
    // let mut count = 0;
    for line in data.lines(){
        let mut line: Vec<_> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        // println!("{line:?}");
        let mut safe = (None, true);
        let mut skip = false;
        let direction = compare(line[0], line[1]);


        let mut pairs = line.windows(2).enumerate();
        while let Some((i, pair)) = pairs.next() {
            let order = compare(pair[0],pair[1]);
            if order == Ordering::Equal || order != direction { 
                if safe.0 == None{
                    safe.0 = Some((i, direction));
                    // line[i+1] = line[i];
                    if pairs.next() == None {break;};
                    continue;
                }
                else{
                    safe.1 = false;
                    break;
                }
            }
        }
        if let Some((i, direction)) = safe.0{
            if let Some(&y) = line.get(i+2){
                let order = compare(line[i], y);
                if order == Ordering::Equal || order != direction { safe.1 = false }
            }
        }
        else { solutions.0 += 1 }
        if safe.1 { solutions.1 += 1 }
    }
    
    println!("{solutions:?}")
}
