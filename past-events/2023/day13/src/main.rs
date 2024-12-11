use std::iter;

use advent_of_code_question::*;
use array2d::Array2D;
use input::{as_array2d, as_mapped_array2d};
use tuples_galore::operate2;

fn main() {
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    let input = input.split("\n\n");
    // println!("{:?}", input);
    'pattern: for pattern in input{
        println!("\n{pattern}\n");
        let pattern = as_mapped_array2d(pattern, |x| x=='#');
        let mut summarized = (0,0);
        let size = (pattern.len(), pattern.get(0).map(|x| x.len()).unwrap_or(0)); // TEMPLATE THIS
        'horizontal: for div_i in 1..size.0{
            println!("i {div_i}");
            let mut smudges = 0;
            for di in 1..=div_i{
                if let  (Some(row1), Some(row2)) = 
                        (pattern.get(div_i - di), pattern.get(div_i+di-1)){
                    smudges += iter::zip(row1.iter(), row2.iter()).filter(|(a,b)| **a==**b).count();
                    if smudges > 1{ continue 'horizontal; }
                }
                
            }
            if smudges==0 { summarized.0 += 100*div_i; }
            else if smudges==1 { summarized.1 += 100*div_i; }
            
            println!("row {}|{}", div_i-1, div_i)
        }
        'vertical: for div_j in 1..size.1{
            println!("j {div_j}");
            let mut smudges = 0;
            for dj in 1..=div_j{
                for i in 0..size.0{
                    if let  (Some(row1), Some(row2)) = 
                    (pattern.get2d((i, div_j - dj)), pattern.get2d((i, div_j+dj-1))){
                        if row1!=row2{
                            smudges += 1;
                            println!("\t smudge dj = {dj} i = {i}");
                            if smudges > 1 {continue 'vertical;}
                            // continue 'vertical;
                        }
                    }
                }
                
            }
            if smudges==0 { summarized.0 += div_j; }
            else if smudges==1 { summarized.1 += div_j; }
            println!("column {}|{}", div_j-1, div_j)
        }
        println!("{summarized:?}");
        // sol.0 += summarized;
        sol = operate2(sol, summarized, |a,b| a+b);
    }

    println!("\n solutions: {sol:?}");
}
