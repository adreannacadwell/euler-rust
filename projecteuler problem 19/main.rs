fn main() -> std::io::Result<()> {
    let mut weekday = 1i32;
    let mut count = 0i32;

    for year in 1900..=2000 {
        for month in 1..=12 {

            if year >= 1901 && weekday == 0 {
                count += 1;
            }

            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 29 } else { 28 }
                }
                _ => unreachable!(),
            };

            weekday = (weekday + days_in_month) % 7;
        }
    }

    println!("{}", count);

    Ok(())
}