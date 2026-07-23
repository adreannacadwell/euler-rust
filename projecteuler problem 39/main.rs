fn 最大公因數(mut a: usize, mut b: usize) -> usize { while b != 0 { let r = a % b; a = b; b = r; }
    a
}
fn main() {
    const LIMIT: usize = 1000;
    let mut 计数 = [0u16; LIMIT + 1];
    let max_m = ((LIMIT / 2) as f64).sqrt() as usize;
    for m in 2..=max_m { for n in 1..m {
        if ((m - n) & 1) == 0 { continue; }
        if 最大公因數(m, n) != 1 { continue; }
        let p0 = 2 * m * (m + n);
        if p0 > LIMIT { break; }
        let mut p = p0;
        while p <= LIMIT { 计数[p] += 1; p += p0;}
    }}
    let mut 最佳_p = 0usize;
    let mut 最大计数 = 0u16;
    for p in 0..=LIMIT { if 计数[p] > 最大计数 { 最大计数 = 计数[p]; 最佳_p = p; } }

    println!("{}", 最佳_p);
}