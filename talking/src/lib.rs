pub fn talking(text: &str) -> &str {
    if text == "" {
        return "Just say something!"
    }
    println!("{}", text);
    let filtered_input = text.chars().filter(|c| c.is_alphabetic());
    let mut is_all_caps = true;

     for c in filtered_input {
        // Check if the character is an uppercase letter
        if !c.is_uppercase() {
            // If not, return false
            is_all_caps = false;
            break;
        }
    }
    if is_all_caps && text.chars().last() == Some('?') {
        return "Quiet, I am thinking!"
    } else if is_all_caps {
        return "There is no need to yell, calm down!"
    } else  if text.chars().last() == Some('?') {
        return "Sure."
    }
    return "Interesting"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", talking("JUST DO IT!"));
        println!("{:?}", talking("Hello how are you?"));
        println!("{:?}", talking("WHAT'S GOING ON?"));
        println!("{:?}", talking("something"));
        println!("{:?}", talking("I LOVE YELLING"));
    }
}
