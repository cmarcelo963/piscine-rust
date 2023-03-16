pub fn first_subword(mut s: String) -> String {
    // Find the first position where a lowercase letter is followed by an uppercase letter,
    // an underscore, or the end of the string.
    let end = s
        .char_indices()
        .skip(1) 
        .find(|(_, c)| c.is_uppercase() || c == &'_')
        .map(|(i, _)| i)
        .unwrap_or_else(|| s.len());

    // Remove everything after the end position and return the result.
    s.truncate(end);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = String::from("helloWorld");
        let s2 = String::from("snake_case");
        let s3 = String::from("CamelCase");
        let s4 = String::from("just");
        let result = first_subword(s1);
        let result1 = first_subword(s2);
        let result2 = first_subword(s3);
        let result4 = first_subword(s4);
        assert_eq!(result, "hello");
    }
}
