fn main() {
    let mut done: bool = false;
    let mut day: i64 = 0;
    let mut day_in_month = 1;
    let mut month = 0;
    let days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let days_per_month_leap = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut year = 1901;

    let mut res = 0;

    while !done {
        let week_day = day%7;
        day_in_month += 1;
        if year % 4 == 0 {
            if day_in_month > days_per_month_leap[month] {
                day_in_month = 1;
                month += 1;
            }
        } else {
            if day_in_month > days_per_month[month] {
                day_in_month = 1;
                month += 1;
            }
        }
        if month > 11 {
            year += 1;
            month = 0;
        }
        if day_in_month == 1 && week_day == 6 {
            res += 1;
            println!("{}: {}-{}-{} wd:{}", day, year, month + 1, day_in_month, week_day);
        }
        if year > 2001 {
            done = true;
        }
        day += 1;
    }
    println!("{:?}", res);
}
