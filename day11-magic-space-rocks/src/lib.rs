pub mod tuples_galore{
    pub fn map2<T,U>(x: (T,T), f: fn(T) -> U) -> (U,U){
        (f(x.0), f(x.1))
    }
    pub fn operate2<T,U,V>(x: (T,T), y: (U,U), f: fn(T,U) -> V) -> (V,V){
        (f(x.0, y.0), f(x.1, y.1))
    }

}

pub mod array2d{
    use crate::tuples_galore::map2;

    pub fn enumerate_array2d<T>(array: &Vec<Vec<T>>) -> Vec<Vec<((usize,usize),&T)>>{
        array.iter().enumerate().map(|(i,row)| row.iter().enumerate().map(|(j,x)| ((i,j),x)).collect()).collect()
    }
    pub fn map_array2d<T,U>(array: &Vec<Vec<T>>, f: fn(&T) -> U) -> Vec<Vec<U>>{
        array.iter().map(|row| row.iter().map(|x| f(x)).collect()).collect()
    }
    pub trait Array2D<T> {
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
}
pub mod input{
    use std::{fmt::Debug, str::FromStr};

    pub fn as_array2d(s: &str) -> Vec<Vec<char>>{
        s.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
    }
    pub fn as_mapped_array2d<T>(s: &str, f: fn(x: char) -> T) -> Vec<Vec<T>>{
        s.lines()
        .map(|l| l.chars()
            .map(f)
            .collect::<Vec<_>>())
        .collect()
    }
    // pattern trait is nightly so multiple functions :(
    pub fn as_vertical_arrays<T: FromStr+Clone, const n: usize>(s: &str, ) -> Vec<Vec<T>> 
    where <T as FromStr>::Err: Debug {
        let mut rows = s.lines();
        let mut out = vec![vec![]; n];
        while let Some(line) = rows.next() {
            for (i,token) in line.split_whitespace().enumerate(){
                if let Some(x) = out.get_mut(i) {
                    x.push(token.parse().expect(&format!("cannot parse item |{}|", token)));
                }
            }
        }
        out
    }
    pub fn as_vertical_arrays_by_str<T: FromStr+Clone,  const n: usize>(s: &str, pattern: &str) -> Vec<Vec<T>> 
    where <T as FromStr>::Err: Debug {
        let mut rows = s.lines();
        let mut out = vec![vec![]; n];
        while let Some(line) = rows.next() {
            for (i,token) in line.split(pattern).enumerate(){
                if let Some(x) = out.get_mut(i) {
                    x.push(token.parse().expect(&format!("cannot parse item |{}|", token)));
                }
            }
        }
        out
    }
    pub fn as_vertical_arrays_by_char<T: FromStr+Clone,  const n: usize>(s: &str, pattern: char) -> Vec<Vec<T>> 
    where <T as FromStr>::Err: Debug {
        let mut rows = s.lines();
        let mut out = vec![vec![]; n];
        while let Some(line) = rows.next() {
            for (i,token) in line.split(pattern).enumerate(){
                if let Some(x) = out.get_mut(i) {
                    x.push(token.parse().expect(&format!("cannot parse item |{}|", token)));
                }
            }
        }
        out
    }
    pub fn as_vertical_arrays_by_chars<T: FromStr+Clone,  const n: usize>(s: &str, pattern: &[char]) -> Vec<Vec<T>> 
    where <T as FromStr>::Err: Debug {
        let mut rows = s.lines();
        let mut out = vec![vec![]; n];
        while let Some(line) = rows.next() {
            for (i,token) in line.split(pattern).enumerate(){
                if let Some(x) = out.get_mut(i) {
                    x.push(token.parse().expect(&format!("cannot parse item |{}|", token)));
                }
            }
        }
        out
    }
    // pub fn as_array2d(s: &str) -> Vec<Vec<char>>
    // pub fn as_array2d(s: &str) -> Vec<Vec<char>>
    // pub fn as_array2d(s: &str) -> Vec<Vec<char>>
}