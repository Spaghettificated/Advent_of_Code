use std::fs;


fn main() {
    // let data = fs::read_to_string("input_test").expect("can't read, need school");
    let data = fs::read_to_string("input").expect("can't read, need school");
    let data: Vec<_> = data.split_whitespace().collect();
    let mut count = 0;
    let (x,y) = (data.len(), data[0].len());

    //XMAS
    let word = "XMAS";
    for i in 0..x{
        for j in 0..y{
            for dx in -1..=1{
                'dircheck: for dy in -1..=1{
                    // println!("");
                    print!("|");
                    for t in 0..4{
                        let ii: i32 = i as i32 + t*dx;
                        let jj: i32 = j as i32 + t*dy;
                        if ii<0 || ii>=x as i32 || jj<0 || jj>=y as i32{
                            continue 'dircheck;
                        }
                        let (ii,jj) = (ii as usize, jj as usize);
                        if data[ii].chars().nth(jj).unwrap() != word.chars().nth(t as usize).unwrap(){
                            continue 'dircheck;
                        }
                        print!("{}",data[ii].chars().nth(jj).unwrap())
                    }
                    count += 1;
                    println!("({i},{j})->({},{})",i as i32+3*dx , j as i32+3*dy )
                }
            }
        }
    }


    //X-MAS
    for i in 1..x-1{
        for j in 1..y-1{
            if data[i].chars().nth(j).unwrap() == 'A'{
                let (a,b,c,d) = (
                    data[i-1].chars().nth(j-1).unwrap(),
                    data[i+1].chars().nth(j+1).unwrap(),
                    data[i-1].chars().nth(j+1).unwrap(),
                    data[i+1].chars().nth(j-1).unwrap(),
                );
                if ((a,b)==('M','S') || (a,b)==('S','M')) && ((c,d)==('M','S') || (c,d)==('S','M')){
                    count += 1;
                    println!("({i},{j})");
                }
            }
        }
    }

    println!("\n{}",count);

    // for i in 0..x{
    //     for j in 0..y{
    //         print!("{}",data[i].chars().nth(j).unwrap())
    //     }
    // println!("")}
}

