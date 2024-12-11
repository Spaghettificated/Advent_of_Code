use std::{fs, iter};
use itertools::Itertools;

fn mul(a:i128, b: i128) -> i128 {a*b}
fn add(a:i128, b: i128) -> i128 {a+b}
fn con(a:i128, b: i128) -> i128 {format!("{a}{b}").parse::<i128>().unwrap()}

fn main() {
    let mut sol = (0,0);
    let filename = "input";
    // let filename = "input-test";
    let data = fs::read_to_string(filename).expect("lost my reading glasses");

    for line in data.lines(){
        let (val,rest) = line.split_once(':').unwrap();
        let val: i128 = val.parse().unwrap();
        let coefs = rest.split_whitespace().map(|s| s.parse::<i128>().unwrap()).collect::<Vec<_>>();
        let nop = coefs.len()-1;
        println!("{val}, {coefs:?}");
        // part 1
        for func_perm in (0..nop).map(|_i| [add,mul]).multi_cartesian_product(){
            let mut solution = coefs[0];
            for (i, x) in coefs[1..coefs.len()].iter().enumerate(){
                solution = func_perm[i](solution,*x)
            }
            if solution==val {sol.0 += val; break;}
        }
        // part 2
        for func_perm in (0..nop).map(|_i| [add,mul,con]).multi_cartesian_product(){
            let mut solution = coefs[0];
            for (i, x) in coefs[1..coefs.len()].iter().enumerate(){
                solution = func_perm[i](solution,*x)
            }
            if solution==val {sol.1 += val; break;}
        }
    }
    println!("{sol:?}");
}
