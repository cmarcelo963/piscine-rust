pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials_list: Vec<String> = Vec::new();
    for name in names {
        let initials = name
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap_or_default().to_ascii_uppercase();
                format!("{}. ", first_char)
            })
            .collect::<String>()
            .trim_end()
            .to_owned();
        initials_list.push(initials);
    }
    initials_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names);
        let answer = ["H. P.", "S. E.", "J. L.", "B. O."];
        assert_eq!(result, answer);
    }
}
