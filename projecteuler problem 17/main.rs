fn letter_count(n: u32) -> usize {
    const ONES: [usize; 20] = [
        0, // 0
        3, // 1 = one
        3, // 2 = two
        5, // 3 = three
        4, // 4 = four
        4, // 5 = five
        3, // 6 = six
        5, // 7 = seven
        5, // 8 = eight
        4, // 9 = nine
        3, // 10 = ten
        6, // 11 = eleven
        6, // 12 = twelve
        8, // 13 = thirteen
        8, // 14 = fourteen
        7, // 15 = fifteen
        7, // 16 = sixteen
        9, // 17 = seventeen
        8, // 18 = eighteen
        8, // 19 = nineteen
    ];

    const TENS: [usize; 10] = [
        0, // 0
        0, // 10 handled above
        6, // 20 = twenty
        6, // 30 = thirty
        5, // 40 = forty
        5, // 50 = fifty
        5, // 60 = sixty
        7, // 70 = seventy
        6, // 80 = eighty
        6, // 90 = ninety
    ];

    match n {
        1..=19 => ONES[n as usize],
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            TENS[tens as usize] + ONES[ones as usize]
        }
        100..=999 => {
            let hundreds = n / 100;
            let rest = n % 100;
            let mut total = ONES[hundreds as usize] + 7; // "x hundred"
            if rest != 0 {
                total += 3; // "and"
                total += letter_count(rest);
            }
            total
        }
        1000 => 11, // "one thousand"
        _ => 0,
    }
}

fn main() {
    let total: usize = (1..=1000).map(letter_count).sum();
    println!("{}", total);
}