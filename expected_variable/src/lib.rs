pub use case;
pub use case::CaseExt;
pub use edit_distance::edit_distance;


pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
    if !string_to_compare.is_camel_lowercase() && !string_to_compare.contains('_'){
        None
    } else {
        let distance  = edit_distance::edit_distance(string_to_compare, expected_string);
        let alikeness = 100 - (distance / expected_string.len() * 100);
        if alikeness > 50 {
            Some(alikeness.to_string() + "%")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    );
    println!(
        "{} close to it",
        expected_variable("soClose", "So_Close").unwrap()
    );
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    );
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    );
    }
}
