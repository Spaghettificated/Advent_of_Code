// my scuffed helper functions

// for treating tuples (and arrays) as knock-off 2D vectors
pub mod tuples_galore{
    use std::ops::{Add, Div, Mul, Neg, Sub};

    pub fn map2<T,U>(x: (T,T), f: fn(T) -> U) -> (U,U){
        (f(x.0), f(x.1))
    }
    pub fn operate2<T,U,V>(x: (T,T), y: (U,U), f: fn(T,U) -> V) -> (V,V){
        (f(x.0, y.0), f(x.1, y.1))
    }
    pub fn neg2<T,V>(x: (T,T)) -> (V,V)             where T: Neg<Output = V>    { map2(x, |x| -x) }
    pub fn add2<T,U,V>(x: (T,T), y: (U,U)) -> (V,V) where T: Add<U, Output = V> { operate2(x, y, |x,y| x + y) }
    pub fn sub2<T,U,V>(x: (T,T), y: (U,U)) -> (V,V) where T: Sub<U, Output = V> { operate2(x, y, |x,y| x - y) }
    pub fn mul2<T,U,V>(x: (T,T), y: (U,U)) -> (V,V) where T: Mul<U, Output = V> { operate2(x, y, |x,y| x * y) }
    pub fn div2<T,U,V>(x: (T,T), y: (U,U)) -> (V,V) where T: Div<U, Output = V> { operate2(x, y, |x,y| x / y) }
    pub fn operate_n<T:Copy, U:Copy, V:Copy, F: Fn(T,U)->V, const N: usize>(a: &[T;N], b: &[U;N], f: F) -> [V;N] {
        let mut out = [None; N];
        for ((o, a), b) in out.iter_mut().zip(a).zip(b){
            *o = Some(f(*a,*b));
        }
        out.map(|x| x.unwrap())
        // a.iter().zip(b).map(|x| x)
    }
    pub fn neg_n<T: Copy, V, const N: usize>(x: &[T;N]) -> [V;N]                           where T: Neg<Output = V>    { x.map(|x| -x) }
    pub fn add_n<T: Copy, U: Copy, V: Copy, const N: usize>(x: &[T;N], y: &[U;N]) -> [V;N] where T: Add<U, Output = V> { operate_n(x, y, |x,y| x + y) }
    pub fn sub_n<T: Copy, U: Copy, V: Copy, const N: usize>(x: &[T;N], y: &[U;N]) -> [V;N] where T: Sub<U, Output = V> { operate_n(x, y, |x,y| x - y) }
    pub fn mul_n<T: Copy, U: Copy, V: Copy, const N: usize>(x: &[T;N], y: &[U;N]) -> [V;N] where T: Mul<U, Output = V> { operate_n(x, y, |x,y| x * y) }
    pub fn div_n<T: Copy, U: Copy, V: Copy, const N: usize>(x: &[T;N], y: &[U;N]) -> [V;N] where T: Div<U, Output = V> { operate_n(x, y, |x,y| x / y) }
}

// for better manipulation of 2d arrays
pub mod array2d{
    use crate::tuples_galore::map2;
    // functions are kinda scuffed as they create new arrays instead of iterators, the trait is so usefull tho
    pub fn enumerate_array2d<T>(array: &Vec<Vec<T>>) -> Vec<Vec<((usize,usize),&T)>>{
        array.iter().enumerate().map(|(i,row)| row.iter().enumerate().map(|(j,x)| ((i,j),x)).collect()).collect()
    }
    pub fn map_array2d<T,U>(array: &Vec<Vec<T>>, f: fn(&T) -> U) -> Vec<Vec<U>>{
        array.iter().map(|row| row.iter().map(|x| f(x)).collect()).collect()
    }
    // for indexing arrays with tuples, checks for bounds, try_... accepts negative numbers (and anthing that try_converts to usize)
    // use with tuple galore module
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
    impl<T> Array2D<T> for [[T]] where [T]: Sized{ // is there a way to do that on all array types? 
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
// for quickly parsing input
pub mod input{
    use std::{fmt::Debug, str::FromStr};
    fn size2d<T>(array: &[Vec<T>]) -> (usize, usize) { (array.len(), array.get(0).map(|x| x.len()).unwrap_or(0)) }
    fn print2d(map: &[Vec<char>]) {
        for row in map{
            for c in row{
                print!("{c}");
            }
            println!();
        }
    }
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
}

pub mod copycat{
    // 'borrowed' from a friend at https://gitlab.com/fili_pk/aoc24/-/blob/master/utils/src/lib.rs?ref_type=heads
    pub struct FindOverlapping<'a, T> {
        t: &'a [T],
        pattern: &'a [T],
        idx: usize,
    }
    
    impl<'a, T: Eq> Iterator for FindOverlapping<'a, T> {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            if self.idx + self.pattern.len().max(1) > self.t.len() {
                return None;
            }
    
            while !self.t[self.idx..].starts_with(self.pattern) {
                self.idx += 1;
                if self.idx + self.pattern.len().max(1) > self.t.len() {
                    return None;
                }
            }
    
            let v = Some(self.idx);
            self.idx += 1;
            v
        }
    }
    
    pub trait FindOverlappingExt<T> {
        fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T>;
    }
    
    impl<T: Eq> FindOverlappingExt<T> for [T] {
        fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T> {
            FindOverlapping {
                t: self,
                pattern,
                idx: 0,
            }
        }
    }
    
    pub struct FindNotOverlapping<'a, T> {
        t: &'a [T],
        pattern: &'a [T],
        idx: usize,
    }
    
    impl<'a, T: Eq> Iterator for FindNotOverlapping<'a, T> {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            if self.idx + self.pattern.len().max(1) > self.t.len() {
                return None;
            }
    
            while !self.t[self.idx..].starts_with(self.pattern) {
                self.idx += 1;
                if self.idx + self.pattern.len().max(1) > self.t.len() {
                    return None;
                }
            }
    
            let v = Some(self.idx);
            self.idx += self.pattern.len();
            v
        }
    }
    
    pub trait FindNotOverlappingExt<T> {
        fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T>;
    }
    
    impl<T: Eq> FindNotOverlappingExt<T> for [T] {
        fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T> {
            FindNotOverlapping {
                t: self,
                pattern,
                idx: 0,
            }
        }
    }
    
}