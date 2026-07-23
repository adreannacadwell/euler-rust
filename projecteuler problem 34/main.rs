fn main() {
    const FAC: [u32; 10] = [
        1,
        1,
        2,
        6,
        24,
        120,
        720,
        5_040,
        40_320,
        362_880,
    ];

    let mut num: u32 = 10;
    let mut total: u32 = 0;
    let mut l: usize;

    'outer: while num < 2540160 {
        let mut num_ = num;
        let mut sum: u32 = 0;

        l = (num.ilog10() + 1) as usize;

        for i in 0..l {
            let divisor = 10u32.pow((l - i - 1) as u32);
            let digit = (num_ / divisor) as usize;
            
            sum += FAC[digit];
            
            if sum > num {
                num -= num_;
                num += divisor * 10;
                continue 'outer; // let's get Rusty!
            }
            
            num_ %= divisor;
        }

        if sum == num {
            println!("{}", num);
            total += num;
        }

        num += 1;   
    }

    println!("{}", total);
}