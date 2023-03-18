pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(5, 2);
        assert_eq!(result, (2, 1));
    }
}
