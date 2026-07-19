fn main() {
    let mut n: u64 = 600_851_475_143;
    let mut largest: u64 = 1;

    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }

    let mut f: u64 = 3;
    while f <= n / f {
        while n % f == 0 {
            largest = f;
            n /= f;
        }
        f += 2;
    }

    if n > 1 {
        largest = n;
    }

    println!("{}", largest);
}