use std::fs;
use day8::my_array2d as a2d;


fn main() {

    // let file = "input";
    let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{:?}", map);
}
