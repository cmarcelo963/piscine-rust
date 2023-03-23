pub fn talking(text: &str) -> &str {
    println!("{}", text);
    if is_all_caps(text) && text.chars().last() == Some('!') {
        return "There is no need to yell, calm down!"
    } else if is_all_caps(text) && text.chars().last() == Some('?') {
        return "Quiet, I am thinking!"
    } else if text.chars().last() == Some('?') {
        return "Sure."
    } else if text == "" {
        return "Just say something!"
    }
    return "Interesting"
}
pub fn is_all_caps(input: &str) -> bool {
    // Filter out non-alphabetic characters
    let filtered_input = input.chars().filter(|c| c.is_alphabetic());

    // Iterate over each character in the filtered string
    for c in filtered_input {
        // Check if the character is an uppercase letter
        if !c.is_uppercase() {
            // If not, return false
            return false;
        }
    }
    // If all characters are uppercase, return true
    true
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
        println!("{:?}", talking(""));
    }
}
