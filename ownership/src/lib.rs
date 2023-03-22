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