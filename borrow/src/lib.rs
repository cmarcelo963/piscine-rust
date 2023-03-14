pub fn str_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "I dont know";
        let result = str_len(str);
        assert_eq!(result, 11);
    }
}
