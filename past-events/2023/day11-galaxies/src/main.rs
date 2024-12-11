use std::fs;

fn main() {
    let mut sol: (i128,i128) = (0,0);
    // let file = "input";
    let file = "input-test";
    let input = fs::read_to_string(file).expect("the unexpected");

    let mut columns = vec![0;input.lines().next().unwrap().len()];
    let mut rows = vec![];

    for (i, row) in input.lines().enumerate(){
        let mut sum = 0;
        for (j,c) in row.chars().enumerate(){
            if c=='#'{
                columns[j] += 1;
                sum+=1;
            }
        }
        if sum>0 {rows.push((i, sum))};
    }

    let columns: Vec<_> = columns.into_iter().enumerate().filter(|&x| x.1!=0).collect();
    let expansions = (2,1000000); 
    
    for di in 1..columns.len(){
        let mut i = 0;
        while let Some(&(b, b_count)) = columns.get(i+di) {
            let (a, a_count) = columns[i];
            let (a,b) = (a as i128, b as i128);
            sol.0 += (expansions.0*(b-a-1) - (expansions.0-1)*(di as i128 - 1) + 1) * a_count * b_count;
            sol.1 += (expansions.1*(b-a-1) - (expansions.1-1)*(di as i128 - 1) + 1) * a_count * b_count;
            i+=1;
        }
    }
    for di in 1..rows.len(){
        let mut i = 0;
        while let Some(&(b, b_count)) = rows.get(i+di) {
            let (a, a_count) = rows[i];
            let (a,b) = (a as i128, b as i128);
            sol.0 += (expansions.0*(b-a-1) - (expansions.0-1)*(di as i128 - 1) + 1) * a_count * b_count;
            sol.1 += (expansions.1*(b-a-1) - (expansions.1-1)*(di as i128 - 1) + 1) * a_count * b_count;
            i+=1;
        }
    }

    println!("\n solutions: {sol:?}");
}
