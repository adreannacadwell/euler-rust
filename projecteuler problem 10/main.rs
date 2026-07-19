fn main() {
    let limit: usize = 2_000_000;
    
    let size = limit / 2;
    let mut composite = vec![false; size];
    composite[0] = true;

    let mut p: usize = 3;
    while p <= limit / p {
        if !composite[p / 2] {
            let mut m = p * p;
            let step = 2 * p;
            while m < limit {
                composite[m / 2] = true;
                m += step;
            }
        }
        p += 2;
    }

    let mut sum: u64 = 2;
    for i in 1..size {
        if !composite[i] {
            sum += (2 * i + 1) as u64;
        }
    }

    println!("{}", sum);
}