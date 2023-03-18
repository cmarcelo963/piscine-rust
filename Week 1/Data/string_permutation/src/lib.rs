use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let s1chars = s1.chars();
    let s2chars = s2.chars();
    let mut s1char_map: HashMap<char, i32> = HashMap::new();
    let mut s2char_map: HashMap<char, i32> = HashMap::new();
    for c in s1chars {
        *s1char_map.entry(c).or_insert(0) += 1;
    }
    for c in s2chars {
        *s2char_map.entry(c).or_insert(0) += 1;
    }
    let mut first_equal = true;
    for (key, value1) in &s1char_map {
        match s2char_map.get(key) {
            Some(value2) if value1 == value2 => {},
            _ => { first_equal = false; break; },
        }
    }
    let mut second_equal = true;
    for (key, value2) in &s1char_map {
        match s1char_map.get(key) {
            Some(value1) if value2 == value1 => {},
            _ => { second_equal = false; break; },
        }
    }
    let all_equal = if first_equal && second_equal {true} else {false};
    all_equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "cde";
	let word1 = "edbca";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
    }
}
