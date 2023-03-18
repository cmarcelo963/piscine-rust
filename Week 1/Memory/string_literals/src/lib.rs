pub fn is_empty(v: &str) -> bool {
    if v == "" {
        return true
    }
    return false
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let (left, right) = v.split_at(index);
    (left, right)
}

pub fn find(v: &str, pat: char) -> usize {
    v.chars().position(|v| v == pat).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "";
        let result = is_empty(str);
        assert_eq!(result, true);
    }
    #[test]
    fn it_works2() {
        let ascii_str = "hello";
        let non_ascii_str = "こんにちは";
        assert_eq!(is_ascii(ascii_str), true);
        assert_eq!(is_ascii(non_ascii_str), false);
    }
    #[test]
    fn it_works3() {
        let input = "hello world";
        let pattern1 = "world";
        let pattern2 = "goodbye";
        assert_eq!(contains(input, pattern1), true);
        assert_eq!(contains(input, pattern2), false);
    }
    #[test]
    fn it_works4() {
        let input = "hello world";
        let index = 5;

        let (left, right) = split_at(input, index);
        println!("Left: {}", left);
        println!("Right: {}", right);
    }
    #[test]
    fn it_works5() {
        let input = "hello world";
        let pattern1 = 'w';
        assert_eq!(find(input, pattern1), 6);
    }
}
