fn main() {
    let mut x: i128 = 143;
    let mut y: i128 = 165;

    let nx = 97 * x + 84 * y - 38;
    let ny = 112 * x + 97 * y - 44;

    x = nx;
    y = ny;

    println!("{}", x*(2*x - 1));
    println!("{}", y*(3*y - 1) / 2);
}