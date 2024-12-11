// use std::env;
use std::{cmp::Ordering, fs};

// fn main() {
//     let data = fs::read_to_string("input").expect("can't read, need school");
//     let (rules,records) = data.split_once("\n\n").unwrap();
//     let rules = rules.lines()
//         .map(|line| line.split_once('|').unwrap())
//         .map(|(x,y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
//     // for rule in &rules {
//     //     println!("|{:?}|", rule);
//     // }
//     let records = records.lines()
//         .map(|line| line.split(',')
//             .map(|x| x.parse::<usize>().unwrap())
//         .collect::<Vec<_>>());
//     // for rule in &records {
//     //     println!("|{:?}|", rule);
//     // }
//     // let mut lookup: &[Option<Vec<usize>>] = &[None;100];
//     let mut lookup = [[false;100];100];
//     for rule in rules{
//         lookup[rule.0][rule.1] = true;
//     }

//     let mut sum = 0;
//     let mut sum_i = 0;
//     let mut incorrect = vec![];
//     'record_check: for record in records{
//         for (i, &x0) in record.iter().enumerate(){
//             if let Some(_) = record[i+1..record.len()].iter().find(|&&x| lookup[x][x0]){
//                 incorrect.push(record);
//                 continue 'record_check;
//             }
//         }
//         let v = record[record.len()/2];
//         sum += v;
//         println!("OK{:?}: {}", record, v);
//     }
//     for mut record in incorrect{
//         record.sort_by(|&a, &b| if lookup[a][b] {Ordering::Greater} else { if lookup[b][a] {Ordering::Less} else {Ordering::Equal} });
//         let v = record[record.len()/2];
//         sum_i += v;
//         println!("!!{:?}: {}", record, v);
//     }
//     println!("\n {sum} :3");
//     println!("\n {sum_i} :3");
// }


fn main() {
    // let input = fs::read_to_string("input.txt").expect("the unexpected");
    // let board = input
    //     .lines()
    //     .map(|l| l.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    // println!("{board:?}");
    println!("lol")
}