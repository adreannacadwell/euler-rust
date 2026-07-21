use std::fs;


fn keep_larger_of_each_pair(nums: &[u16]) -> Vec<u16> {
    nums.windows(2)
        .map(|pair| pair[0].max(pair[1]))
        .collect()
}


// === Main ===
fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("triangle.txt")?;
    let mut numbers: Vec<u16> = vec![0; 100];

    for line in content.lines().rev() {
        let nums: Vec<u16> = line
            .split_whitespace()
            .map(|s| s.parse::<u16>().expect("invalid number"))
            .collect();

        for (x, y) in numbers.iter_mut().zip(nums.iter()) {
            *x += *y;
        }
        
        if numbers.len() > 1 {
            numbers = keep_larger_of_each_pair(&numbers);
        } else {
            println!("{}", numbers[0]);
            break;
        }
    }

    Ok(())
}