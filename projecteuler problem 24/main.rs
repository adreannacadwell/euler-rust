fn factorial(n: usize) -> u64 {
    const FACT: [u64; 10] = [
        1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880,
    ];

    FACT[n]
}

fn main() {
    let mut limit: u64 = 1_000_000 - 1;
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut answer: u64 = 0;

    for _ in 0..10 {
        let f = factorial(digits.len() - 1);
        let c = (limit / f) as usize;

        let x = digits.remove(c) as u64;
        answer = answer * 10 + x;

        limit %= f;
    }

    println!("{answer}");
}