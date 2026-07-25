#[link(name = "euler23", kind = "static")]
extern "C" {
    fn solve_non_abundant_sums() -> u64;
}

fn main() {
    let answer = unsafe { solve_non_abundant_sums() };
    println!("{answer}");
}