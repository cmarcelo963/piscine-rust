use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let number_of_days_in_year = if NaiveDate::from_ymd_opt(year, 1, 1).is_some() {366} else {365};

    if (number_of_days_in_year / 2) == 0 {
        None
    } else {
        Some(NaiveDate::from_ymd(year, 7, 2).weekday())
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
