fn som(limit: u64, k: u64) -> u64 {
    // Sum of multiples
    let n = (limit - 1) / k;
    k * n * (n + 1) / 2
}

// === MAIN ===
fn main() {
    let l = 1000;

    let s3 = som(l, 3);
    let s5 = som(l, 5);
    let s15 = som(l, 15);

    let result = s3 + s5 - s15;

    println!("{}", result);
}