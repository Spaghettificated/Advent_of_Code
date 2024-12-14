// use advent_of_code_question::*;

use advent_of_code_question::tuples_galore::map2;




fn main() {
    let mut sol = (0,0);
    let part2 = true;
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");
    let machines = input.split("\n\n");
    // let mut euler = [true;101];
    let mut euler = [true;1000000];
    
    let mut primes = vec![];
    for i in 2..euler.len(){
        if euler[i]{
            primes.push(i as i128);
            let mut x = i*i;
            while x < euler.len() {
                euler[x] = false;
                x += i;
            }
        }
    }
    println!("primes: {primes:?}");

    for machine in machines{
        // println!("{}", machine);
        let mut lines = machine.lines();
        let a = lines.next().unwrap();
        let a: Vec<i128> = a.split(", ").map(|s| s[s.find("+").unwrap()+1..].parse().unwrap()).collect();
        let b = lines.next().unwrap();
        let b: Vec<i128> = b.split(", ").map(|s| s[s.find("+").unwrap()+1..].parse().unwrap()).collect();
        let prize = lines.next().unwrap();
        let prize: Vec<i128> = prize.split(", ").map(|s| s[s.find("=").unwrap()+1..].parse().unwrap()).collect();
        let  a = (a[0],a[1]);
        let  b = (b[0],b[1]);
        let mut prize = (prize[0],prize[1]);
        if part2 { prize = map2(prize, |x| 10000000000000+x)}
        println!("{:?}", [a,b,prize]);

        let mut eq = ((a.0, b.0, prize.0), (a.1, b.1, prize.1));

        let mut swapped = false;
        while eq.0.0 != 0{
            // let (d,r) = (a.0.div_euclid(a.1), a.0.rem_euclid(a.1));
            if eq.0.0 < eq.1.0{
                eq = (eq.1, eq.0);
                swapped = !swapped;
                continue;
            }
            let d = eq.0.0 / eq.1.0;
            eq.0.0 -= d * eq.1.0;
            eq.0.1 -= d * eq.1.1;
            eq.0.2 -= d * eq.1.2;
        }
        println!("\n{:?}\n{:?}\n", eq.0, eq.1);
        if eq.0.2 % eq.0.1 != 0 { continue;}
        
        let b = eq.0.2 / eq.0.1;
        eq.1.2 -= eq.1.1 * b;
        eq.1.1 = 0;
        println!("{:?}\n{:?}\n", eq.0, eq.1);
        for &p in primes.iter(){
            while eq.0.1 % p == 0 && eq.0.2 % p ==0 {
                eq.0.1 /= p;
                eq.0.2 /= p;
            }
        }
        println!("{:?}\n{:?}\n", eq.0, eq.1);
        let b = eq.0.2 / eq.0.1;
        if eq.1.2 % eq.1.0 != 0 { continue;}
        let a = eq.1.2 / eq.1.0;

        println!("presses {a} {b}\n---------------------------------");
        sol.0 += 3*a+b;
        // let min = operate2(a, b, i128::min);
        // let da = sub2(a, min);
        // let db = sub2(b, min);
    }


    println!("\n solutions: {sol:?}");
}
