fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    let mut num_prod: u32 = 1;
    let mut den_prod: u32 = 1;

    for n in 10..100 {
        for d in (n + 1)..100 {
            let n_tens = n / 10;
            let n_ones = n % 10;
            let d_tens = d / 10;
            let d_ones = d % 10;

            // Skip trivial cases with 0 in the ones place
            if n_ones == 0 && d_ones == 0 {
                continue;
            }

            // Case 1: cancel n_tens with d_tens
            if n_tens == d_tens && d_ones != 0 {
                if n * d_ones == d * n_ones {
                    num_prod *= n_ones;
                    den_prod *= d_ones;
                }
            }

            // Case 2: cancel n_tens with d_ones
            if n_tens == d_ones && d_tens != 0 {
                if n * d_tens == d * n_ones {
                    num_prod *= n_ones;
                    den_prod *= d_tens;
                }
            }

            // Case 3: cancel n_ones with d_tens
            if n_ones == d_tens && d_ones != 0 {
                if n * d_ones == d * n_tens {
                    num_prod *= n_tens;
                    den_prod *= d_ones;
                }
            }

            // Case 4: cancel n_ones with d_ones
            if n_ones == d_ones && d_tens != 0 {
                if n * d_tens == d * n_tens {
                    num_prod *= n_tens;
                    den_prod *= d_tens;
                }
            }
        }
    }

    let g = gcd(num_prod, den_prod);
    println!("{}", den_prod / g);
}