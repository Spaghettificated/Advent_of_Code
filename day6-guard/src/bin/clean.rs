use std::{fs, ops::Neg};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Wards{
    Forwards,
    Backwards,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction{
    Horizontal(Wards),
    Vertical(Wards),
}
use Wards::{Forwards,Backwards};
use Direction::{Horizontal,Vertical};

impl Neg for Wards { type Output = Wards; fn neg(self) -> Self::Output { if self==Forwards {Backwards} else {Forwards}} }

// fn turn(v: (i32, i32))->(i32, i32){
//     let (y,x) = v;
//     return (x, -y)
// }
fn turn(d: Direction)->Direction{
    match d {
        Horizontal(wards) => {},
        Vertical(wards) => todo!(),
    }
}

fn main() {
    // let directions = (0..4).into_iter().map(|x| {let mut v = (-1, 0); for _ in 0..x {v = turn(v)} v} ).collect::<Vec<_>>();
    let input = fs::read_to_string("input").expect("the unexpected");
    // let input = fs::read_to_string("input-test").expect("the unexpected");
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let size = (map.len(), map[0].len());
    let mut rows: Vec<Vec<_>> = input.lines().map(|line| line.match_indices('#').map(|x| x.0).collect()).collect();
    // let mut columns: Vec<Vec<_>> = (0..size.0).map(|i| map.iter().map(|row| row[i]).collect()).collect();
    let mut columns: Vec<Vec<_>> =  map.iter().enumerate().map(|(i, row)| row.iter().enumerate().filter(|&x| *x.1=='#').map(|x| x.0).collect()).collect();
    let mut sol = (1,0);
    let mut pos = map.iter()
        .enumerate()
        .map(|(i,row)| row.iter().position(|&c| c=='^').map(|x| [i,x]))
        .reduce(|acc, el| el.or(acc)).expect("empty input").expect("no player");
    // map.iter().
    println!("{:?}", columns);
    let tables = [rows, columns];
    let directions: [(usize, bool); 4] = [(1, true),(0, false),(1, false),(0, true)];
    let mut directions = directions.iter().cycle();
    loop {
        let &(table_id, reversed) = directions.next().unwrap();
        let delta = if reversed {-1} else {1};
        let i = pos[table_id];
        let line = tables[table_id][i].clone().into_iter().map(|x| x as i32);
        let line: Vec<i32> = if reversed {line.rev().collect()} else {line.collect()};
        let i = pos[1-table_id] as i32;
        let new_pos_try = line.get(line.binary_search(&i).err().unwrap() + 1);
        match new_pos_try {
            Some(new_pos) => {
                let new_pos = *new_pos - delta;
                sol.0 += new_pos - i;
                pos[1-table_id] = new_pos as usize;
            },
            None => {
                break;
            },
        }
    }
}