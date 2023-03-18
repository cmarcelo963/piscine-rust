pub fn to_url(s: &str) -> String {
    let mut output = String::new();

    for c in s.chars() {
        if c == ' ' {
            output.push_str("%20");
        } else {
            output.push(c);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        let result = to_url(s);
        assert_eq!(result, "Hello,%20world!".to_string());
    }
}
