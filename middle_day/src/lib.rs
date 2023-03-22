use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let number_of_days_in_year = if NaiveDate::from_ymd_opt(year, 1, 1).is_some() {365} else {366};
    println!("{}", number_of_days_in_year);
    if (number_of_days_in_year / 2) == 0 {
        Some(NaiveDate::from_ymd(year, 7, 2).weekday())
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{:?}", middle_day(1892).unwrap());

    }
}
