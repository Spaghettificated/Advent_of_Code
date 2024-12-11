use std::fs;

use itertools::{Itertools, Powerset};

// fn prod(a:i128, b: i128, n: usize){
//     let x = (0..=b);
//     // let x = (0..=b).collect::<Vec<_>>();
//     for _ in 0..n{
//         let x = x.multi_cartesian_product();
//     }
// }

fn mul(a:i128, b: i128) -> i128 {a*b}
fn add(a:i128, b: i128) -> i128 {a+b}
fn con(a:i128, b: i128) -> i128 {format!("{a}{b}").parse::<i128>().unwrap()}

fn main() {
    let mut sol = (0,0);
    let data = fs::read_to_string("input").expect("lost my reading glasses");
    // let data = fs::read_to_string("input-test").expect("lost my reading glasses");
    for line in data.lines(){
        let (val,rest) = line.split_once(':').unwrap();
        let val: i128 = val.parse().unwrap();
        let coefs = rest.split_whitespace().map(|s| s.parse::<i128>().unwrap()).collect::<Vec<_>>();
        let funcs = [con, mul, add];
        let nf = funcs.len();
        let nop = coefs.len()-1;
        println!("{val}, {coefs:?}");
        for perm in (0..nop).map(|_i| 0..nf).multi_cartesian_product(){
            let mut new_coefs = vec![coefs[0]];
            // for (i, x) in coefs[1..coefs.len()].iter().enumerate(){
            //     // println!("\t\t {i} {x} {} {} {:?}", perm[i], new_coefs[i], new_coefs);
            //     if perm[i]==1{
            //         let last = new_coefs.pop().unwrap();
            //         new_coefs.push(last*x);
            //     }
            //     else{
            //         new_coefs.push(*x);
            //     }
            // }
            for (i, x) in coefs[1..coefs.len()].iter().enumerate(){
                
                // println!("\t\t {i} {x} {} {:?}", perm[i], new_coefs);
                let last = new_coefs.pop().unwrap();
                new_coefs.push(funcs[perm[i]](last,*x));
            }
            let sum: i128 = new_coefs.iter().sum();
            // println!("\t{perm:?}\t{new_coefs:?}\t{sum}");
            if sum==val {sol.0 += val; break;}
            // if sum==val {sol.0 +=1}
            // if sum==val {sol.0 += val}
        }
    }
    println!("{sol:?}")
    
}
