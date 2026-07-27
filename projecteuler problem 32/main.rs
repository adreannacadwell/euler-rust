use std::collections::HashSet;

const TARGET_MASK: u16 = 0b11_1111_1110; // bits 1..9 set, bit 0 unused

fn digit_mask(mut n: u32) -> Option<u16> {
    let mut mask: u16 = 0;

    while n > 0 {
        let d = n % 10;
        if d == 0 {
            return None; // zero is not allowed
        }

        let bit = 1u16 << d;
        if mask & bit != 0 {
            return None; // repeated digit inside the same number
        }

        mask |= bit;
        n /= 10;
    }

    Some(mask)
}

fn is_pandigital(a: u32, b: u32, p: u32) -> bool {
    let ma = match digit_mask(a) {
        Some(m) => m,
        None => return false,
    };
    let mb = match digit_mask(b) {
        Some(m) => m,
        None => return false,
    };
    let mp = match digit_mask(p) {
        Some(m) => m,
        None => return false,
    };

    let combined = ma | mb | mp;

    combined == TARGET_MASK
        && ma.count_ones() + mb.count_ones() + mp.count_ones() == 9
}

fn main() {
    let mut products = HashSet::new();

    // 1-digit × 4-digit = 4-digit
    for a in 1..=9 {
        for b in 1234..=9876 {
            let p = a * b;
            if p >= 1000 && is_pandigital(a, b, p) {
                products.insert(p);
            }
        }
    }

    // 2-digit × 3-digit = 4-digit
    for a in 12..=98 {
        for b in 123..=987 {
            let p = a * b;
            if p >= 1000 && is_pandigital(a, b, p) {
                products.insert(p);
            }
        }
    }

    let sum: u32 = products.iter().sum();
    println!("{}", sum);
}