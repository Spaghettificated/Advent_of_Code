// use std::env;
use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("can't read, need school");
    // let data: Vec<(i32,i32)> = data.split('\n')
    //     .filter(|x| !x.is_empty())
    //     .map(|x| x.split_at(5))
    //     .map(|(x1,x2)| (x1.parse::<i32>().unwrap(), x2.trim().parse::<i32>().unwrap()))
    //     .collect();
    let mut count = 0;
    for line in data.split('\n').filter(|s| !s.is_empty()){
        let line: Vec<_> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        if line.len() < 2{
            continue;
        }
        println!("{line:?}");
        let mut safe = false;
        
        for i in 0..line.len(){
            let spliced = [&line[0..i], &line[i+1..line.len()] ].concat();
            safe = true;
            let asc = spliced[1] > spliced[0];
            for pair in spliced.windows(2){
                // let (x0,x1) = (pair[0],pair[1]);
                if pair.len()<2{
                    continue;
                }
                let diff = pair[1] - pair[0];
                match diff {
                    d if d>0 && d<=3  && asc => {}
                    d if d<0 && d>=-3 && !asc => {}
                    _ => {safe = false;}
                }
            }
            if safe{
                break;
            }
        }
        if safe {count += 1;}
        // line.chunks(2).reduce(|a, x| )
    }
    println!("{count}")
}
