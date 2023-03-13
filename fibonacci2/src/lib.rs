pub fn fibonacci(n: u32) -> u32 {
    if n <= 0 {
          return 0;
    } else if n == 1{
          return 1;
} else {
    return fibonacci(n-1)  + fibonacci(n-2);
 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(20);
        assert_eq!(result, 6765);
    }
}
