// use advent_of_code_question::*;

use std::collections::HashMap;


fn main() {
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");

    let stones: Vec<String> = input.split_whitespace().map(|x| x.to_owned()).collect(); 
    let mut stone_count: HashMap<String, u128> = HashMap::new();
    for stone in stones.iter(){ // we assume all inpput stones are unique for now (im lazy)
        stone_count.insert(stone.to_owned(), 1);
    }

    let mut next: HashMap<String, (String, Option<String>)> = HashMap::new();
    next.insert("0".to_owned(), ("1".to_owned(), None));

    // let mut stones: Vec<String> = input.split_whitespace().collect();
    println!("{:?}", stones);

    for it in 0..214 {
        let stones_iter: Vec<_> = stone_count.keys().map(|x| x.to_owned()).collect();
        let mut to_insert: Vec<(String, u128)> = vec![];
        for stone in stones_iter{
            let count = *stone_count.get(&stone).unwrap();
            // let mut next_stone;
            // let mut next_stone2 = None;
            if let Some((s0,s_opt)) = next.get(&stone) {
                // next_stone = s.clone();
                to_insert.push((s0.clone(), count));
                if let Some(s1) = s_opt {to_insert.push((s1.clone(), count));};
            }
            else if stone.len()%2==0{
                let (s0,s1) = stone.split_at(stone.len()/2);
                // next_stone = s0.to_string();
                to_insert.push((s0.to_string(), count));
                if let Some(i) = s1.chars().position(|c| c!='0'){
                    // next_stone2 = Some(s1[i..].to_string());
                    to_insert.push((s1[i..].to_string(), count));
                }
                else{
                    // next_stone2 = Some("0".to_string());
                    to_insert.push(("0".to_string(), count));
                }
            }
            else { 
                let n = stone.parse::<u128>().unwrap();
                // next_stone =  (n*2024).to_string();
                to_insert.push(((n*2024).to_string(), count));
            }
            stone_count.insert(stone, 0);

        }
        for (s,c) in to_insert{
            if let Some(count) = stone_count.get_mut(&s) {
                *count += c;
            }
            else {
                stone_count.insert(s, c);

            }
        }

        if it == 24{
            sol.0 = stone_count.iter().map(|(_,c)| c).sum();
        }
        if it == 74{
            sol.1 = stone_count.iter().map(|(_,c)| c).sum();
        }

        // println!("{:?}", stone_count.iter().filter(|(s,c)| **c>0).collect::<Vec<_>>());
        // if it %5==0{
        //     println!("{it} {}", stones.len());
        // }
    }
    // sol.0 = stones.len();
    // sol.1 = stone_count.iter().map(|(_,c)| c).sum();

    println!("\n solutions: {sol:?}");
}
