pub fn arrange_phrase(phrase: &str) -> String {
    // Split the input phrase into words
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words based on the number in each word
    words.sort_by_key(|word| word.chars().next().unwrap_or('0'));

    // Join the words back together with spaces
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
