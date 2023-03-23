pub fn rotate(input: &str, key: i8) -> String {
    println!("{}",input);
    let mut result: String = String::new();
    for char in input.chars() {
        if char.is_ascii_alphabetic() {
            if char >= 'a' && char <= 'z' {
                let mut rotated_ascii = (char as i16 + key as i16) % 123;
                if key < 0 {
                    rotated_ascii = if rotated_ascii < 97 {rotated_ascii + 26} else {rotated_ascii};
                    result.push(rotated_ascii as u8 as char)

                } else {
                    rotated_ascii = if rotated_ascii < 97 {rotated_ascii + 97} else {rotated_ascii};
                    result.push(rotated_ascii as u8 as char)
                }
            } else {
                let mut rotated_ascii = (char as i16 + key as i16) % 91;
                if key < 0 {
                    rotated_ascii = if rotated_ascii < 65 {rotated_ascii + 26} else {rotated_ascii};
                    result.push(rotated_ascii as u8 as char)
                }else {
                    rotated_ascii = if rotated_ascii < 65 {rotated_ascii + 26} else {rotated_ascii};
                    result.push(rotated_ascii as u8 as char)
                }
            }
        } else {
            result.push(char);
        }
    }
    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("The letter \"a\" becomes: {}", rotate("a", 26));
        // println!("The letter \"m\" becomes: {}", rotate("m", 0));
        // println!("The letter \"m\" becomes: {}", rotate("m", 13));
        // println!("The letter \"a\" becomes: {}", rotate("a", 15));
        // println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
        // println!(
        //     "The decoded message is: {}",
        //     rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
        // );
        // println!(
        //     "The decoded message is: {}",
        //     rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
        // );
        // println!(
        //     "Your cypher wil be: {}",
        //     rotate("Testing with numbers 1 2 3", 4)
        // );
        // println!("Your cypher wil be: {}", rotate("Testing", -14));
        // println!("The letter \"a\" becomes: {}", rotate("a", -1));      
    }
}
