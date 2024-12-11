use std::fs;

fn main() {
    let mut sol = (0,0);
    let file = "input";
    // let file = "input-test";
    let s_input = fs::read_to_string(file).expect("the unexpected");
    let input: Vec<_> = s_input.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let input_pairs: Vec<_> = input.chunks(2).collect();
    let mut disc2: Vec<_> = input_pairs.iter().enumerate().map(|(i,x)| (*x.get(0).unwrap_or(&0), *x.get(1).unwrap_or(&0), i)).collect();
    let mut disc1 = vec![];
    // part 1
    for (i, pair) in input_pairs.iter().enumerate() {
        if let Some(&n) = pair.get(0) {
            disc1.append(&mut vec![Some(i);n]);
        }
        if let Some(&n) = pair.get(1) {
            disc1.append(&mut vec![None;n]);            
        }
    }
    println!("{:?}", disc1.iter().map(|x| x.map(|d| d.to_string()).unwrap_or(".".to_string()) ).collect::<String>());
    while let Some(i) = disc1.iter().position(|x| x.is_none()) {
        if disc1.len()<=i{
            break;
        }
        let mut x = disc1.pop().unwrap();
            while x.is_none() && disc1.len()>i{
                x = disc1.pop().unwrap();
        }
        if let Some(y) = disc1.get_mut(i){
            *y = x;
        }
        else {
            break;
        }
    }
    // part2
    let n = disc2.len();
    // let mut once_more = true;
    println!("{:?}",disc2);
    // let mut max_j = 0;
    // let to_insert = vec![];
    // let mut first = 1;
    // let mut attempted = vec![false; n]; 

    // READ THE PUZZLE CAREFULLY!!! didn't read to attempt to move file only once and in what order, wasted a lot of time

    // 'main: loop {
    //     // println!("{:?}",disc2);
    //     'first: for i in (1..n).rev(){
    //         // if i<max_j {break;}
    //         // let i = disc2.iter().position(|(_,_,))
    //         if attempted[disc2[i].2]{
    //             continue 'first; 
    //         }
    //         for j in (0..i){
    //             if disc2[i].0 <= disc2[j].1{
    //                 println!("moving {}", disc2[i].2);
    //             // if disc2[i].0 <= disc2[j].1 {
    //                 disc2[i-1].1 += disc2[i].0 + disc2[i].1;
    //                 (disc2[j].1 ,disc2[i].1) = (0, disc2[j].1 - disc2[i].0);
    //                 let x = disc2.remove(i);
    //                 disc2.insert(j+1, x);
    //                 // n-=1;
    //                 attempted[disc2[i].2] = true;
    //                 continue 'main;
    //                 // max_j = max_j.max(j);
    //                 // continue 'first;
    //             }
    //         }
    //         attempted[disc2[i].2] = true;
    //     }
    //     // if once_more {once_more=false; continue;}
    //     // else {break;}
    //     break;
    // }

    for k in (1..n).rev(){
        let i = disc2.iter().position(|(_,_,x)| *x==k).unwrap();
        for j in 0..i{
            if disc2[i].0 <= disc2[j].1{
                println!("moving {}", disc2[i].2);
                if j!=i-1{
                    disc2[i-1].1 += disc2[i].0 + disc2[i].1;
                    (disc2[j].1 ,disc2[i].1) = (0, disc2[j].1 - disc2[i].0);
                }
                else {
                    (disc2[j].1 ,disc2[i].1) = (0, disc2[j].1 + disc2[i].1);
                }
                let x = disc2.remove(i);
                disc2.insert(j+1, x);
                break;
            }
        }
    }
    
    let mut disc2_flat = vec![];
    for pair in disc2.iter(){
            disc2_flat.append(&mut vec![Some(pair.2); pair.0]);
            disc2_flat.append(&mut vec![None; pair.1]);            
    }
    sol.0 = disc1.iter().enumerate().map(|(i,x)| i * x.unwrap_or(0)).sum();
    sol.1 = disc2_flat.iter().enumerate().map(|(i,x)| i * x.unwrap_or(0)).sum();
    // sol.1 = disc2_flat.iter().map(|(i,x)| i * x.unwrap_or(0)).sum();
    // println!("{s_input:?}");
    // println!("{input:?}");
    // println!("{input_pairs:?}");
    println!("--------");
    println!("{:?}", disc1.iter().map(|x| x.map(|d| d.to_string()).unwrap_or(".".to_string()) ).collect::<String>());
    println!("{:?}", disc2_flat.iter().map(|x| x.map(|d| d.to_string()).unwrap_or(".".to_string()) ).collect::<String>());
    // println!("{:?}", disc2_flat.iter().map(|(i,x)| x.map(|d| d.to_string()).unwrap_or("#".to_string()) ).collect::<String>());
    println!("{sol:?}");
}
