use std::fs;

fn main() {
    let contents = fs::read_to_string("input.csv").unwrap();

    let mut cols: Vec<u16> = vec![0; 50];

    for line in contents.lines() {
        let value = line.trim().trim_end_matches(',');

        for (i, b) in value.bytes().enumerate() {
            cols[i] += (b - b'0') as u16;
        }
    }

    let mut sum: u64 = 0;
    for n in &cols[..10] {
        sum += *n as u64;
        sum *= 10;
    }
    
    let mut i: u64 = 1;
    for n in &cols[10..] {
        sum += *n as u64;
        if sum % i + 900 < i {
            break;
        } else {
            i *= 10;
        }
        sum *= 10;
    }

    // => I hate Rust :(
    println!("{}", sum);
}