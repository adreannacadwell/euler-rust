fn main() {
    let limit = 10_000;
    let mut sums = vec![0usize; limit + 1];

    for d in 1..=limit / 2 {
        let mut multiple = d * 2;
        while multiple <= limit {
            sums[multiple] += d;
            multiple += d;
        }
    }

    let mut total = 0usize;

    for a in 2..limit {
        let b = sums[a];
        if b != a && b <= limit && sums[b] == a {
            total += a;
        }
    }

    println!("{}", total);
}