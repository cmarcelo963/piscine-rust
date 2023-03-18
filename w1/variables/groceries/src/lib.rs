pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    let result = &vec[index];
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
		"yogurt".to_string(),
		"panettone".to_string(),
		"bread".to_string(),
		"cheese".to_string(),
	];
        insert(&mut groceries, String::from("nuts"));
        let result = at_index(&mut groceries, 1);
        assert_eq!(result, "panettone");
    }
}
