pub fn arrange_phrase(phrase: &str) -> String {
    let mut words = phrase.split_whitespace().collect::<Vec<&str>>();
    words.sort_by_key(|word| {
        word.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
    });
    return words.join(" ")
        .chars()
        .filter(|c| !c.is_digit(10))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let phrase = "is2 Thi1s T4est 3a";
        let result = arrange_phrase(phrase);
        assert_eq!(result, "This is a Test");
    }
}
