use core::num;
use std::{fs, iter};

fn main() {
    let mut sol = (0,0);
    // let file = "input-test";
    let file = "input";
    let data= fs::read_to_string(file).expect("c'mon, it'll work");
    for line in data.lines() {
        let (state_str, numbers) = line.split_once(' ').unwrap();
        let state: Vec<_> = state_str.chars().collect();
        let mut numbers: Vec<usize> = numbers.split(',').map(|c| c.parse().unwrap()).collect();
        let mut groups: Vec<(Option<char>, Vec<usize>)> = vec![];
        let mut new_group: (Option<char>, Vec<usize>) = (None, vec![]);
        let mut last_c = '.';
        for &c in state.iter().chain(iter::once(&'.')){
            if c == '.' {
                if new_group.0 != None{
                    groups.push((new_group.0, new_group.1.drain(..).collect()));
                    new_group = (None, vec![]);
                }
            }
            else {
                match new_group.0 {
                    Some(_) => {
                        if c==last_c {*new_group.1.last_mut().unwrap() += 1}
                        else { new_group.1.push(1); }
                    },
                    None => {new_group = (Some(c), vec![1])},
                }
                last_c = c;
            }
        }
        let groups: Vec<_> = groups.into_iter().map(|(c,x)| (c.unwrap(), x)).collect();
        // let mut groups_iter = groups.iter();
        // let mut possibilities: Vec<(usize, Option<Vec<usize>>)> = vec![(0,None)];
        // numbers.reverse();
        let mut possibilities = vec![(0,groups[0].clone(), numbers.clone())];
        possibilities[0].2.reverse();
        possibilities[0].1.1.reverse();
        let sums: Vec<usize> = groups.iter().map(|(_, v)| v.iter().sum()).collect();
        // let count: usize = sums.iter().sum();
        let mut count = 0;
        println!("{} {:?} ->  {:?}  {:?}", state_str, numbers, sums, groups);
        println!("\t-----------------------------------------");
        loop {
            // for p in &possibilities { println!("\t{p:?}")}
            if let Some((mut i, mut group, mut nums)) = possibilities.pop(){
                println!("\t{i},  {}{:?}{:w$} {:?}", group.0, group.1, "", nums, w = 50-group.1.len()*3  - if group.1.is_empty() {2} else {0});
                if group.1.is_empty(){
                    i += 1;
                    if i<groups.len(){
                        let mut new_group = groups[i].clone();
                        new_group.1.reverse();
                        possibilities.push((i, new_group, nums));
                    }
                    else {
                        if nums.is_empty(){
                            count += 1;
                            println!("\t\t*");
                        }
                        else { println!("\t\tx") }
                    }
                }
                else{
                    if let Some(n) = nums.pop(){
                        if group.0 == '#' {
                            let ng = group.1.pop().unwrap(); // earlier checked if group.1 is not empty
                            if n==ng { 
                                if let Some(x) = group.1.last_mut() {
                                    *x-=1;
                                }
                                possibilities.push((i, ('?', group.1), nums)); 
                            }
                            else if n>ng { 
                                // let nng_opt = group.1.last().map(|x| *x);
                                if let Some(&nng) = group.1.last(){
                                    if n>ng+nng{ // enough to fill bot '#' and '?' fields
                                        group.1.pop(); // poppin '?'
                                        nums.push(n-ng-nng);
                                        possibilities.push((i, group, nums)); 
                                    }
                                    else if n==ng+nng{
                                        group.1.pop(); // poppin '?'
                                        if group.1.last() == None{
                                            possibilities.push((i, group, nums)); 
                                        }
                                        else { println!("\t\tx") }
                                    }
                                    else{ // enough to fill '#' but not '?'
                                        *group.1.last_mut().unwrap() -= n-ng +1;
                                        possibilities.push((i, ('?', group.1), nums)); 
                                    }
                                }
                                else { println!("\t\tx") }
                            }
                            else { println!("\t\tx") } // if n<ng there's not enough space to fit next num
                        }
                        else if group.0 == '?'{
                            let ng = group.1.pop().unwrap(); // earlier checked if group.1 is not empty
                            if ng > 0{ // co jeśli pierwsze to kropka
                                println!("\t\t..");
                                let mut new_group = group.clone();
                                new_group.1.push(ng-1);
                                let mut new_nums = nums.clone();
                                new_nums.push(n);
                                possibilities.push((i, new_group, new_nums));
                            }

                            if n > ng { 
                                if group.1.last().is_some() {
                                    nums.push(n-ng);
                                    possibilities.push((i,('#', group.1), nums));
                                }
                                else if ng==0 {
                                    nums.push(n);
                                    possibilities.push((i,group,nums));
                                }
                                else { println!("\t\tx") }
                            }
                            else if n == ng {
                                if group.1.last()==None {possibilities.push((i,('#', group.1),nums));}
                                else {println!("\t\tx");}
                                // else {println!("\t\tx {:?}", nums.last());}
                            }
                            else if n+1 == ng {
                                possibilities.push((i,('#', group.1),nums));
                            }
                            else if n+1 < ng {
                                group.1.push(ng-n-1);
                                possibilities.push((i,group,nums));
                            }
                            // if n >= ng{
                            //     println!("\t\t..");
                            //     for j in n-ng..n{
                            //         let mut new_group = ('#', group.1.clone());
                            //         new_group.1.push(j);
                            //         possibilities.push((i, new_group, nums.clone()));
                            //     }
                            // }

                            // if n==ng { 
                            //     possibilities.push((i, ('#', group.1), nums)); 
                            //     nums.push(n);
                            // }
                            // else if n>ng { 
                            //     nums.push(n-ng);
                            //     possibilities.push((i, ('#', group.1), nums)); 
                            // }
                            // else if n<ng{
                            //     println!("\t\t..");
                            //     // for i in 1..=ng-n{
                            //     //     let mut new_group = group.clone();
                            //     //     new_group.1.push(i);
                            //     //     possibilities.push((i, new_group, nums.clone()));
                            //     // }
                            //     // if let None = group.1.last(){
                            //     //     possibilities.push((i, ('#', group.1), nums));
                            //     // }
                            //     for j in 1..=ng-n-1{
                            //         let mut new_group = group.clone();
                            //         new_group.1.push(j);
                            //         possibilities.push((i, new_group, nums.clone()));
                            //     }
                            //     possibilities.push((i, ('#', group.1.clone()), nums.clone()));
                            //     if let None = group.1.last(){
                            //         possibilities.push((i, ('#', group.1), nums));
                            //     }
                            // }
                        }
                    }
                    else{
                        if group.0=='?'{
                            group.1.pop();
                            group.0 = '#';
                            possibilities.push((i, group, nums));
                        }
                        else if group.0=='#'{
                            println!("\t\tx");
                        }
                    }
                }
            }
            else {
                break;
            }

        }
        println!("\t-----------------------------------------");
        sol.0 += count;
        println!("->{count}")
    }
    println!("{sol:?}")
}
