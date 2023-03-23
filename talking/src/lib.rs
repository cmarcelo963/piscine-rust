pub fn talking(text: &str) -> &str {
    println!("Print the string: ||{}half", text);
    if text.is_empty() || text.chars().all(|c| c.is_ascii_whitespace()) {
        return "Just say something!"
    }
    let filtered_input = text.chars().filter(|c| c.is_alphabetic());
    let mut is_all_caps = false;

    for c in filtered_input {
        // Check if the character is an uppercase letter
        if !c.is_uppercase() {
            // If not, return false
            is_all_caps = false;
            break;
        } else {
            is_all_caps = true;
        }
    }
    if is_all_caps && text.chars().last() == Some('?') {
        return "Quiet, I am thinking!"
    } else if text.chars().last() == Some('?') {
        return "Sure."
    } else if is_all_caps {
        return "There is no need to yell, calm down!"
    } else  if text.chars().last() == Some('?') {
        return "Sure."
    } else {
        return "Interesting"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("{:?}", talking("JUST DO IT!"));
        // println!("{:?}", talking("Hello how are you?"));
        // println!("{:?}", talking("WHAT'S GOING ON?"));
        // let the_string: String = Default::default();
        // println!("{:?}", talking("7?"));
        println!("{:?}", talking("        "));
    }
}
