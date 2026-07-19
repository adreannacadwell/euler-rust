fn main() {
    let n: u128 = 254_000;
    let m = n / 2; // number of odd square numbers among 1^2..n^2

    // sum of first m odd squares:
    // 1^2 + 3^2 + 5^2 + ... + (2m-1)^2 = m(2m-1)(2m+1)/3
    let sum = m * (2 * m - 1) * (2 * m + 1) / 3;

    println!("{}", sum);
}