// use advent_of_code_question::*;

fn main() {
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    // let input = input;
    println!("{:?}", input);

    println!("\n solutions: {sol:?}");
}
