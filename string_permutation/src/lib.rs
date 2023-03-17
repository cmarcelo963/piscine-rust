use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
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
    let mut all_equal = true;
    for (key, value1) in &s1char_map {
        match s2char_map.get(key) {
            Some(value2) if value1 == value2 => {},
            _ => { all_equal = false; break; },
        }
    }
    all_equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
	let word1 = "thougth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
    }
}
