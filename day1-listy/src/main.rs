// use std::env;
use std::{fs, iter::zip};

fn main() {
    let data = fs::read_to_string("input").expect("can't read, need school");
    let data: Vec<(i32,i32)> = data.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.split_at(5))
        .map(|(x1,x2)| (x1.parse::<i32>().unwrap(), x2.trim().parse::<i32>().unwrap()))
        .collect();
    let mut l0: Vec<i32> = data.iter().map(|x| x.0).collect();
    let mut l1: Vec<i32> = data.iter().map(|x| x.1).collect();
    l0.sort();
    l1.sort();
    let mut sum = 0;
    // for (x1,x2) in zip(l0, l1){
    //     sum += (x2-x1).abs();
    // }
    let (mut i, mut j) = (0,0);
    let mut count = 0;
    while j<l1.len() {
        if l0[i] == l1[j]{
            count += 1;
            j += 1;
        }
        else{
            if l0[i] < l1[j] {
                sum += l0[i]*count;
                i += 1;
                count = 0;
            }
            else {
                j+=1;
            }
        }

    }
    println!("{sum}")
}
