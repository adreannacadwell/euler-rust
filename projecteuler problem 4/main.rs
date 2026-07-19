fn is_palindrome(n: u32) -> bool {
    let mut x = n;
    let mut rev = 0u32;

    while x > 0 {
        rev = rev * 10 + (x % 10);
        x /= 10;
    }

    rev == n
}

// === Main ===
fn main() {
    let mut best = 0u32;

    for a in (100u32..=999).rev() {
        if a * 999 <= best {
            break;
        }

        for b in (a..=999u32).rev() {
            let prod = a * b;

            if prod <= best {
                break;
            }

            if is_palindrome(prod) {
                best = prod;
                break;
            }
        }
    }

    println!("{}", best);
}