pub fn stars(n: u32) -> String {
    match u32::checked_pow(2, n) {
        Some(pow) => "*".repeat(pow as usize),
        None => "".to_string(),
    }
    // let mut counter = 1;
    // let mut number_of_stars: u64= 2;
    // let mut result = String::new();
    // loop { 
    //     if counter != n {
    //         number_of_stars *= 2;
    //         counter += 1;
    //     } else {
    //         break
    //     }
    // }
    // for _ in 0..number_of_stars {
    //     result += "*"
    // }
    // result
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
