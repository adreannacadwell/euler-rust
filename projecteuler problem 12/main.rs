fn main() {
    let limit: usize = 1000000;
    
    let mut div  = vec![0; limit +1];

    for i in 1..=limit {
        for j in (i..=limit).step_by(i) {
            div[j] += 1;
        }
    }

    let mut n: usize = 1;
    let mut total: u16;
    loop {
        
        if n%2 == 0 {
            total = div[n / 2] * div[n + 1];
        } else {
            total = div[n] * div[(n + 1) / 2];
        }
        
        if total > 500 {
            println!("{}", n * (n + 1) / 2);
            break;
        }

        n += 1;
    }
}