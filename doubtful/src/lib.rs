pub fn doubtful(s: &mut String) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("Hello");
        doubtful(&mut s);
        assert_eq!(s, "Hello?".to_string());
    }
}
