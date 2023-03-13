pub fn rev_str(input: &str) -> String {
    let initial = input.to_string();
    let result = initial.chars().rev().collect();
    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("cat");
        assert_eq!(result,"tac");
    }
}
