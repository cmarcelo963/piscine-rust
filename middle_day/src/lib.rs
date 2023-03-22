use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let leap_year = year % 4 == 0 && (year % 100 == 0 || year % 400 == 0);
    let days_in_year = if leap_year {366} else {365};
    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_day_point = ((days_in_year - 1) / 2) + 1;
        let middle_day = NaiveDate::from_ymd(year, 1, 1).checked_add_signed(chrono::Duration::days(middle_day_point as i64 - 1));
        middle_day.map(|d| d.weekday())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", middle_day(1022).unwrap());
    }
}
