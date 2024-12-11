use std::fs;

fn main() {
    // let data = fs::read_to_string("input_test").expect("can't read, need school");
    let data = fs::read_to_string("input").expect("can't read, need school");
    let data: Vec<_> = data.split_whitespace().collect();
    let mut count = 0;
    let (x_size,y_size) = (data.len() as i32, data[0].len() as i32);

    let word = "XMAS";
    let w_len = word.len() as i32;
    for i in 0..x_size{    for j in 0..y_size{
        for dx in -1..=1{    'dircheck: for dy in -1..=1{
            // print!("|");
            for t in 0..w_len{
                let (ii,jj) = (i + t*dx, j + t*dy);
                let c= 
                    usize::try_from(ii).ok()
                    .and_then(|i| data.get(i))
                    .and_then(|&line| usize::try_from(jj).ok()
                        .and_then(|j|line.chars().nth(j)))
                    .filter(|&c| c == word.chars().nth(t as usize).unwrap() );
                if c.is_none() {continue 'dircheck;}
                // let c = if let Some(l) = line { 
                //     usize::try_from(jj).ok().and_then(|j| l.chars().nth(j))
                // } else {continue 'dircheck;};

                // let ii = if let Ok(x) = usize::try_from(ii)    {x} else {continue 'dircheck;};
                // let jj = if let Ok(x) = usize::try_from(jj)    {x} else {continue 'dircheck;};
                // let c = if let Some(x) = data.get(ii).and_then(|line| line.chars().nth(jj))    {x} else {continue 'dircheck;};
                // if c != word.chars().nth(t as usize).unwrap() {continue 'dircheck;}
            }
            count += 1;
            println!("{:?}",vec![1,2,3].geti(-1));
            println!("({i},{j})->({},{})",i as i32+3*dx , j as i32+3*dy )
        }}
    }}
    println!("\n{count}")
}

trait GetExt<T> {
    fn geti(&self, i: i32) -> Option<&T>;
}

impl<T> GetExt<T> for [T] {
    fn geti(&self, i: i32) -> Option<&T> {
        if i < 0 {
            return None;
        }

        self.get(i as usize)
    }
}