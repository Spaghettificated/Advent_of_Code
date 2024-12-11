use std::{fs, usize};

fn get<T: Clone>(map: &Vec<Vec<T>>, pos: (usize,usize)) -> T{
    return map[pos.0][pos.1].clone();
}
fn get_i32<T: Clone>(map: &Vec<Vec<T>>, pos: (i32,i32)) -> T{
    let pos = (pos.0 as usize, pos.1 as usize);
    return map[pos.0][pos.1].clone();
}
fn get_mut<T>(map: &mut Vec<Vec<T>>, pos: (usize,usize)) -> &mut T{
    &mut map[pos.0][pos.1]
}
fn get_mut_i32<T>(map: &mut Vec<Vec<T>>, pos: (i32,i32)) -> &mut T{
    let pos = (pos.0 as usize, pos.1 as usize);
    &mut map[pos.0][pos.1]
}
fn in_bounds(pos: (i32,i32), size: (usize,usize))->bool{
    pos.0 >= 0 &&  pos.1 >= 0 && (pos.0 as usize) < size.0  && (pos.1 as usize) < size.1 
}
fn add_touple(a: (i32,i32),b: (i32,i32))-> (i32,i32){
    (a.0+b.0, a.1+b.1)
}


fn main() {
    let symbols = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let mut sol = (0,0);
    let file = "input";
    // let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut antenas = vec![];
    for (i, row) in map.iter().enumerate(){
        for (j,&c) in row.iter().enumerate(){
            antenas.push((c, (i as i32,j as i32)));
        }
    }

    let size = (map.len(), map[0].len());
    let mut antinodes: Vec<Vec<bool>> = vec![ vec![false; size.1]; size.0];
    let mut antinodes_p2: Vec<Vec<bool>> = vec![ vec![false; size.1]; size.0];

    for symbol in symbols{
        let same_antenas = antenas.iter().filter(|(c,_)| *c==symbol).collect::<Vec<_>>();
        for (i,(_, p1)) in same_antenas.iter().enumerate(){
            for (_, p2) in same_antenas.iter().skip(i){

                if p1 != p2 {
                    let a1 = ( 2*p2.0-p1.0, 2*p2.1-p1.1 );
                    let a2 = ( 2*p1.0-p2.0, 2*p1.1-p2.1 );
                    if in_bounds(a1, size){
                        *get_mut_i32(&mut antinodes, a1) = true;
                    }
                    if in_bounds(a2, size){
                        *get_mut_i32(&mut antinodes, a2) = true;
                    }

                    let d1 = ( p2.0-p1.0, p2.1-p1.1 );
                    let d2 = ( p1.0-p2.0, p1.1-p2.1 );
                    let (mut x1,mut x2) = (*p1,*p2);
                    while in_bounds(add_touple(x1, d1), size) {
                        x1 = add_touple(x1, d1);
                        *get_mut_i32(&mut antinodes_p2, x1) = true;
                        // println!("{x1:?}");
                    }
                    while in_bounds(add_touple(x2, d2), size) {
                        x2 = add_touple(x2, d2);
                        *get_mut_i32(&mut antinodes_p2, x2) = true;
                    }
                }

            }
        }
    }
    for row in antinodes{
        sol.0 += row.iter().filter(|x| **x).count();
    }
    for row in antinodes_p2{
        sol.1 += row.iter().filter(|x| **x).count();
    }
    println!("{:?}", sol)
}
