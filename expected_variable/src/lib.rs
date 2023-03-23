pub use case;
pub use case::CaseExt;
pub use edit_distance::edit_distance;

pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
    let string_lowercase = &string_to_compare.to_ascii_lowercase();

    if (string_lowercase == &string_to_compare.to_lowercase()
        || &string_lowercase.to_camel_lowercase() == string_lowercase)
        && !string_lowercase.contains('-')
        && !string_lowercase.contains(' ')
    {
        let distance = edit_distance(&string_lowercase, &expected_string.to_lowercase());
        println!("{}", distance);
        if distance == 0 {
            return Some("100%".to_string());
        }
        let mut alikeness = 100;
        if distance * 100 / expected_string.len() > 100 {
            alikeness = 0;
        } else {
            alikeness = 100 - (distance * 100 / expected_string.len());
        }
        println!(
            "{}, {}, {}, {}",
            alikeness, distance, expected_string, string_to_compare
        );
        println!("{}", Some(alikeness.to_string() + "%").unwrap());
        if alikeness >= 50 {
            Some(alikeness.to_string() + "%")
        } else {
            None
        }
    } else {
        None
    }
}

// pub fn expected_variable(string_to_compare: &str, expected_string: &str) -> Option<String> {
//     // !string_to_compare.is_camel_lowercase() &&
//     // !string_to_compare.contains('_')
//     println!("String to compare: {}, expected string: {}", string_to_compare, expected_string);
//     if !string_to_compare.is_camel_lowercase() && !string_to_compare.contains('_'){
//         println!("Its failing: {}", string_to_compare.is_camel_lowercase());
//         None
//     } else {
//         let distance  = edit_distance::edit_distance(&string_to_compare.to_lowercase(), &expected_string.to_lowercase()) as f32;
//         println!("String to compare: {}, String expected: {}", string_to_compare, expected_string);
//         let expected_length = expected_string.len() as f32;
//         let alikeness = (1.0 - (distance / expected_length)) * 100.0;
//         println!("{}, {}, {}", alikeness, distance, expected_string.len());
//         if alikeness > 50.0 {
//             Some(format!("{:.2}%", alikeness.ceil() as u32))
//         } else {
//             None
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{} close to it",
            expected_variable("On_Point", "on_point").unwrap()
        );
        // println!(
        //     "{} close to it",
        //     expected_variable("soClose", "So_Close").unwrap()
        // );
        // println!(
        //     "{:?}",
        //     expected_variable("something", "something_completely_different")
        // );
        // println!(
        //     "{} close to it",
        //     expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        // );
    }
}
