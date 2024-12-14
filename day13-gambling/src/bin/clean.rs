


fn operator<T:Copy, U:Copy, V:Copy, F: Fn(T,U)->V, const N: usize>(a: &[T;N], b: &[U;N], f: F) -> [V;N] {
    let mut out = [None; N];
    for ((o, a), b) in out.iter_mut().zip(a).zip(b){
        *o = Some(f(*a,*b));
    }
    out.map(|x| x.unwrap())
    // a.iter().zip(b).map(|x| x)
}

fn main(){
    let mut sol = (0,0);
    let file = std::env::args_os().nth(1).map(|s| if s == "i" {"input".to_owned()} else {s.into_string().unwrap()}).unwrap_or("input-test".to_owned());
    let input = std::fs::read_to_string(file).expect("the unexpected");

    for machine in input.split("\n\n"){
        let mut eq: [[i128; 4]; 2] = [[0;4];2];
        let mut it = machine.match_indices(&['=','+']);
        for i in 0..3{
            for j in 0..2{
                let s = &machine[it.next().unwrap().0 ..];
                eq[j][i] = s[1 .. s.find(&[',','\n']).unwrap_or(s.len())].parse().expect(s);
            }
        }
        for j in 0..2{
            eq[j][3] = eq[j][2] + 10000000000000;
        }
        println!("\n{:?}\n{:?}",eq[0],eq[1]);
        while eq[0][0] != 0 {
            if eq[1][0] > eq[0][0]{
                eq.reverse();
            }
            let d = eq[0][0] / eq[0][0];
            eq[0] = operator(&eq[0], &eq[1], |e0,e1| e0 - d * e1 );
        }
        
        println!("\n{:?}\n{:?}",eq[0],eq[1]);
        eq = [eq[0].map(|x| -x*eq[1][1]), eq[1].map(|x| -x*eq[0][1])];
        eq[1] = operator(&eq[1], &eq[0], |e1,e0| e1 - e0) ;
        let [mut a,mut b] = [[0,0],[0,0]];
        let (b0, a0) = (eq[0][1], eq[1][0]);
        for i in 0..2{
            if eq[0][2+i] % b0 != 0 || eq[1][2+i] % a0 != 0 { continue; }
            b[i] = eq[0][2+i] / b0;
            a[i] = eq[1][2+i] / a0;
        }

        println!("\n{:?}\n{:?}",eq[0],eq[1]);
        println!("\nb={:?}\na={:?}",b,a);
        println!("-----------------");
        sol.0 += 3*a[0]+b[0];
        sol.1 += 3*a[1]+b[1];
    }
    println!("\nsolutions: {sol:?}")
}