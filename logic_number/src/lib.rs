pub fn number_logic(num: u32) -> bool {
    let num_in_str_len = num.to_string().len() as u32;
    let num_in_str = num.to_string();
    let mut result = 0;
    for char in num_in_str.chars() {
        result += char.to_digit(10).expect("test").pow(num_in_str_len)
    }
    println!("{}", result);
    if result == num {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let array = [9, 10, 153, 154];
        // for pat in &array {
        //     if number_logic(*pat) == true {
        //         println!(
        //             "this number returns {} because the number {} obey the rules of the sequence",
        //             number_logic(*pat),
        //             pat
        //         )
        //     }
        //     if number_logic(*pat) == false {
        //         println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        //     }
        // }
        println!("{}", number_logic(10))
    }
}
