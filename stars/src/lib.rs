pub fn stars(n: u32) -> String {
    match u32::checked_pow(2, n) {
        Some(pow) => "*".repeat(pow as usize),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", stars(1));
        println!("{}", stars(4));
        println!("{}", stars(200));
    }
}
