pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(6);
        assert_eq!(result, 720);
    }
}
