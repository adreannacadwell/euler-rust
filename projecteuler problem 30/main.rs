fn main() {
    const POWER5: [u32; 10] = [
        0,
        1,
        32,
        243,
        1_024,
        3_125,
        7_776,
        16_807,
        32_768,
        59_049,
    ];

    let mut num: u32 = 1000;
    let mut total: u32 = 0;

    'outer: while num < 354294 {
        let mut num_ = num;
        let mut sum: u32 = 0;

        for i in 0..6 {
            let divisor = 10u32.pow(5 - i);
            let digit = (num_ / divisor) as usize;
            
            sum += POWER5[digit];
            
            if sum > num {
                num -= num_;
                num += divisor * 10;
                continue 'outer; // let's get Rusty!
            }
            
            num_ %= divisor;
        }

        if sum == num {
            total += num;
        }

        num += 1;   
    }

    println!("{}", total);
}