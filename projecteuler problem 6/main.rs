fn main() {
    let n = 100;
    let sum = n * (n + 1) / 2;
    let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;
    let answer = sum * sum - sum_of_squares;
    println!("{answer}");
}