use std::{arch::x86_64, fs, iter, usize};

// 2d array tuple indexing toolkit
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


fn main() {
    let symbols = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let mut sol = (0,0);
    let file = "input";
    // let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");
    let map = str_to_array2d(&input);
    let mut antenas = vec![];
    for (i, row) in map.iter().enumerate(){
        for (j,&c) in row.iter().enumerate(){
            if c != '.' { antenas.push((c, (i as i32,j as i32))); }
        }
    }

    let size = (map.len(), map[0].len());
    let mut antinodes: (Vec<Vec<bool>>, Vec<Vec<bool>>) = (vec![ vec![false; size.1]; size.0], vec![ vec![false; size.1]; size.0]);

    for symbol in symbols{
        let same_antenas = antenas.iter().filter(|(c,_)| *c==symbol).collect::<Vec<_>>();
        for (i,(_, p1)) in same_antenas.iter().enumerate(){
            for (_, p2) in same_antenas.iter().skip(i){
                let (&p1, &p2) = (p1,p2);
                if p1 != p2 {
                    //part 1
                    let a1 = operate2(p2, p1, |a, b| 2*b - a);
                    let a2 = operate2(p1, p2, |a, b| 2*b - a);
                    // if let Some(x) = try_get2d_mut(&mut antinodes.0, a1) {
                    if let Some(x) = antinodes.0.try_get2d_mut(a1) {
                        *x = true;
                    }
                    if let Some(x) = try_get2d_mut(&mut antinodes.0, a2) {
                        *x = true;
                    }

                    //part 2
                    let (mut x1, mut x2) = (p1,p2);
                    let d1 = operate2(p1, p2, |a,b| a-b);
                    let d2 = operate2(p2, p1, |a,b| a-b);
                    while let Some(x) = try_get2d_mut(&mut antinodes.1, x1) {
                        *x = true;
                        x1 = operate2(x1, d1, |a,b| a+b);
                    }
                    while let Some(x) = try_get2d_mut(&mut antinodes.1, x2) {
                        *x = true;
                        x2 = operate2(x2, d2, |a,b| a+b);
                    }
                }

            }
        }
    }
    for row in antinodes.0{
        sol.0 += row.iter().filter(|x| **x).count();
    }
    for row in antinodes.1{
        sol.1 += row.iter().filter(|x| **x).count();
    }
    println!("{:?}", sol)
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