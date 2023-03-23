pub fn is_pangram(s: &str) -> bool {
    let lowercase_of_string = &s.to_lowercase();
    for char in 'a'..'z' {
        if !lowercase_of_string.contains(char) {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            is_pangram("the quick brown fox jumps over the lazy dog!")
        );
        println!("{}", is_pangram("this is not a pangram!"));
    }
}
