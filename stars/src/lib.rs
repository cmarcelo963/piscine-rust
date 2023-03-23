pub fn stars(n: u32) -> String {
    let mut counter = 1;
    let mut number_of_stars = 2;
    let mut result = String::new();
    println!("{}", n);
    loop { 
        if counter != n {
            number_of_stars *= 2;
            counter += 1;
        } else {
            break
        }
    }
    for _ in 0..number_of_stars {
        result += "*"
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", stars(1));
        println!("{}", stars(4));
        println!("{}", stars(5));
    }
}
