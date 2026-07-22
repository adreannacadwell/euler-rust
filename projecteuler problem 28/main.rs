fn main() {
    let n: u32 = (1001 + 1) / 2;

    let c1: u32 = (n * (n + 1) * (2*n + 1)) / 6;
    let c2: u32 = (n * (n + 1)) / 2;
    let c3: u32 = n;

    let sum = 16*c1 - 28*c2 + 16*c3 - 3;

    println!("{}", sum);
}