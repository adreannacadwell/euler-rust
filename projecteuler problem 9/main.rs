fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn find_triplet_with_sum(s: u64) -> Option<(u64, u64, u64)> {
    let limit = ((s / 2) as f64).sqrt() as u64 + 1;

    for m in 2..=limit {
        for n in 1..m {
            // m and n must be one even, one odd
            if (m - n) % 2 == 0 {
                continue;
            }
            // and coprime
            if gcd(m, n) != 1 {
                continue;
            }

            let denom = 2 * m * (m + n);
            if s % denom == 0 {
                let k = s / denom;

                let a = k * (m * m - n * n);
                let b = k * (2 * m * n);
                let c = k * (m * m + n * n);

                let mut triplet = [a, b, c];
                triplet.sort_unstable();
                return Some((triplet[0], triplet[1], triplet[2]));
            }
        }
    }

    None
}

fn main() {
    let sum = 1000;

    match find_triplet_with_sum(sum) {
        Some((a, b, c)) => {
            println!("Triplet: ({}, {}, {})", a, b, c);
            println!("Product: {}", a * b * c);
        }
        None => {
            println!("No Pythagorean triplet found for sum {}", sum);
        }
    }
}