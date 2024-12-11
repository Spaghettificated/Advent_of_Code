use std::{collections::HashMap, fs};

// use itertools::Itertools;
fn str_to_array2d(s: &str) -> Vec<Vec<char>>{
    s.lines()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect()
}
fn enumerate_array2d<T>(array: &Vec<Vec<T>>) -> Vec<Vec<((usize,usize),&T)>>{
    array.iter().enumerate().map(|(i,row)| row.iter().enumerate().map(|(j,x)| ((i,j),x)).collect()).collect()
}
fn map_array2d<T,U>(array: &Vec<Vec<T>>, f: fn(&T) -> U) -> Vec<Vec<U>>{
    array.iter().map(|row| row.iter().map(|x| f(x)).collect()).collect()
}
fn map2<T,U>(x: (T,T), f: fn(T) -> U) -> (U,U){
    (f(x.0), f(x.1))
}
fn operate2<T,U,V>(x: (T,T), y: (U,U), f: fn(T,U) -> V) -> (V,V){
    (f(x.0, y.0), f(x.1, y.1))
}
fn get2d<T>(array: &Vec<Vec<T>>, pos: (usize,usize)) -> Option<&T>{
    let row = array.get(pos.0)?;
    row.get(pos.1)
}
fn try_get2d<T, I: TryInto::<usize> >(array: &Vec<Vec<T>>, pos: (I,I)) -> Option<&T>{
    let pos = map2(pos, |x| x.try_into().ok());
    let pos = (pos.0?, pos.1?);
    get2d(array, pos)
}
fn get2d_mut<T>(array: &mut Vec<Vec<T>>, pos: (usize,usize)) -> Option<&mut T>{
    let row = array.get_mut(pos.0)?;
    let x = row.get_mut(pos.1)?;
    Some(x)
}
fn try_get2d_mut<T, I: TryInto::<usize> >(array: &mut Vec<Vec<T>>, pos: (I,I)) -> Option<&mut T>{
    let pos = map2(pos, |x| x.try_into().ok());
    let pos = (pos.0?, pos.1?);
    get2d_mut(array, pos)
}

// enum Pipe{
//     Horizontal,
//     Vertical,
//     Joint((i32, i32)),
// }
// use Pipe::*;
// impl Pipe {
//     fn 
//     fn from_char(c: char)->Option<Self> {
//         match c {
//             '-' => Some(Horizontal),
//             '|' => Some(Vertical),
//             'F' => Some(Joint((0,-1))),
//             '7' => Some(Joint((0,-1))),
//             'J' => Some(Joint((0,-1))),
//             'L' => Some(Joint((0,-1))),
//             _=>None,
//         }
//     }
//     fn move_through(&self, dir: (i32,i32)) -> Option<(i32,i32)> {
//         match self {
//             Horizontal => if dir.1==0 {Some(dir)} else {None},
//             Vertical => if dir.0==0 {Some(dir)} else {None},
//             Joint(d) => {
//                 if d
//             },
//         }
//     }
// }
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
struct Pipe{
    dir_in: (i32, i32),
    dir_out: (i32, i32),
}
impl Pipe {
    fn turn(dir: (i32,i32))->(i32,i32){
        (-dir.1, dir.0)
    }
    fn from_char(c: char)->Option<Self> {
        match c {
            '|' => Some(Pipe { dir_in: (1,0),  dir_out: (1,0) , }),
            '-' => Some(Pipe { dir_in: (0,1),  dir_out: (0,1) , }),
            'F' => Some(Pipe { dir_in: (0,-1), dir_out: Self::turn((0,-1)), }),
            '7' => Some(Pipe { dir_in: (-1,0), dir_out: Self::turn((-1,0) ), }),
            'J' => Some(Pipe { dir_in: (0,1),  dir_out: Self::turn((0,1) ), }),
            'L' => Some(Pipe { dir_in: (1,0),  dir_out: Self::turn((1,0)), }),
            _=>None,
        }
    }
    fn move_through(&self, dir: (i32,i32)) -> Option<(i32,i32)> {
        if dir==self.dir_in { Some(self.dir_out) }
        else {
            if dir == map2(self.dir_out, |x| -x) {
                Some(map2(self.dir_in, |x| -x)) 
            }
            else { None }
        }
    }
}
fn main() {
    let mut pipes : HashMap<char, (i32,i32)> = HashMap::new();

    let mut sol = (0,0);
    let file = "input";
    // let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");
    // let map = str_to_array2d(&input);
    let char_map = str_to_array2d(&input);
    let map: Vec<Vec<_>> = input.lines()
        .map(|l| l.chars()
            .map(|c| Pipe::from_char(c))
            .collect::<Vec<_>>())
        .collect();

    let size = (map.len(), map[0].len());
    let mut animal = None;
    for (i,row) in char_map.iter().enumerate(){
        println!("{row:?}");
        animal = animal.or(row.iter().position(|x| *x=='S').map(|j| (i as i32, j as i32)) )
    }
    let animal = animal.unwrap();
    let mut paths: Vec<_> = (0..4)
        .map(|x| {let mut d = (0,1); for _ in 0..x {d=Pipe::turn(d)} d})
        .map(|dir| Some((animal, dir)))
        .collect();
    let mut dist = 1;
    // 'main: for _ in (1..10){
    'main: loop{
        println!("{dist}: {:?}", paths);
        for path_opt in paths.iter_mut(){
            // *path_opt = path_opt
            //     .map(|(pos, dir)| {
            //         let new_pos = 
            //     })

            if let Some((pos, dir)) = path_opt{
                *pos = operate2(*pos, *dir, |a,b| a+b);
                if *pos == animal{
                    sol.0 = dist/2;
                    break 'main;
                }

                if let Some(&Some(pipe)) = map.try_get2d(*pos){
                    if let Some(d) = pipe.move_through(*dir){
                        *dir = d;
                    }
                    else { *path_opt = None; println!("\t can't move through pipe")}
                }
                else { *path_opt = None; println!("\t no pipe in front"); }
            } 
            // if !ok { *path_opt = None };
        }
        dist += 1;
    }
    println!("\n solutions: {sol:?}");
}



trait Array2D<T> {
    fn get2d(&self, pos: (usize,usize)) -> Option<&T>;
    fn get2d_mut(&mut self, pos: (usize,usize)) -> Option<&mut T>;
    fn try_get2d<I: TryInto::<usize> >(&self, pos: (I,I)) -> Option<&T>{
        let pos = map2(pos, |x| x.try_into().ok());
        let pos = (pos.0?, pos.1?);
        self.get2d(pos)
    }
    fn try_get2d_mut<I: TryInto::<usize> >(&mut self, pos: (I,I)) -> Option<&mut T>{
        let pos = map2(pos, |x| x.try_into().ok());
        let pos = (pos.0?, pos.1?);
        self.get2d_mut(pos)
    }
}
impl<T> Array2D<T> for [[T]] where [T]: Sized{
    fn get2d(&self, pos: (usize,usize)) -> Option<&T> {
        let row = self.get(pos.0)?;
        row.get(pos.1)        
    }
    fn get2d_mut(&mut self, pos: (usize,usize)) -> Option<&mut T> {
        let row = self.get_mut(pos.0)?;
        row.get_mut(pos.1)        
    }
}
impl<T> Array2D<T> for [Vec<T>] {
    fn get2d(&self, pos: (usize,usize)) -> Option<&T> {
        let row = self.get(pos.0)?;
        row.get(pos.1)        
    }
    fn get2d_mut(&mut self, pos: (usize,usize)) -> Option<&mut T> {
        let row = self.get_mut(pos.0)?;
        row.get_mut(pos.1)        
    }
}