use std::fs;

use itertools::Itertools;
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
    let dirs = [(0,1),(1,0),(0,-1),(-1,0)];
    let mut sol = (0,0);
    let file = "input";
    // let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");
    // let map = str_to_array2d(&input);
    let map: Vec<Vec<_>> = input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>())
        .collect();

    let size = (map.len(), map[0].len());
    let mut trailheads = vec![];
    for row in &map{
        println!("{row:?}");
    }
    for row in enumerate_array2d(&map){
        // println!("{row:?}");
        for (pos, &height) in row{
            if height==0 {trailheads.push(map2(pos, |x| x as i32));}
        }
    }
    let mut scores = vec![];
    // let mut trails: Vec<_> = trailheads.iter().map(|x| vec![x]).collect();
    for trailhead in &trailheads{
        println!("starting from {trailhead:?}");
        let mut score = (0,0);
        let mut peaks = vec![];
        // let mut checked: Vec<((i32, i32), Option<i32>)> = vec![];
        let mut to_check = vec![(trailhead.clone(), 0)];

        while let Some((pos, height)) = to_check.pop() {
            // println!("\t{height}");
            for &dir in &dirs{
                let p = operate2(pos, dir, |a,b| a+b);
                // if checked.iter().find(|(x,_)| *x==p).is_some(){
                //     continue;
                // }
                if let Some(&h) = map.try_get2d(p){
                    // checked.push((p, Some(h)));
                    if h == height+1{
                        if h==9{
                            score.1+=1;
                            peaks.push(p);
                        }
                        else {
                            to_check.push((p,h));
                        }
                    }
                }
                // else{
                //     checked.push((p, None));
                // }
            }
        }
        score.0 = peaks.iter().unique().count();
        println!("score: {score:?}");
        scores.push((trailhead, score));
        sol.0+=score.0;
        sol.1+=score.1;
    }
    
    println!("\nscores:");
    for s in scores{
        println!("\t{s:?}");
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