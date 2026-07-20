fn collatz_next(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    const LIMIT: usize = 1_000_000;

    let mut cache = vec![0u32; LIMIT];
    cache[1] = 1;

    let mut best_start = 1usize;
    let mut best_len = 1u32;

    for start in 2..LIMIT {
        let mut n = start as u64;
        let mut path: Vec<u64> = Vec::new();

        while n >= LIMIT as u64 || cache[n as usize] == 0 {
            path.push(n);
            n = collatz_next(n);
        }

        let mut length = if n < LIMIT as u64 {
            cache[n as usize]
        } else {
            0
        };

        while let Some(x) = path.pop() {
            length += 1;

            if x < LIMIT as u64 {
                cache[x as usize] = length;
            }
        }

        let start_len = cache[start];
        if start_len > best_len {
            best_len = start_len;
            best_start = start;
        }
    }

    println!("{} {}", best_start, best_len);
}