pub fn pig_latin(text: &str) -> String {
    fn is_vowel(c: char) -> bool {
        match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            }
    }
    let mut new_string = text.to_string();
    loop {
        for char in text.chars() {
            if !is_vowel(char) {
                new_string.remove(0);
                new_string.push(char);
            } else {
                break
            }
        }
        break
    }
    new_string.push('a');
    new_string.push('y');
    return new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", pig_latin(&String::from("igloo")));
        println!("{}", pig_latin(&String::from("apple")));
        println!("{}", pig_latin(&String::from("hello")));
        println!("{}", pig_latin(&String::from("square")));
        println!("{}", pig_latin(&String::from("xenon")));
        println!("{}", pig_latin(&String::from("chair")));
        println!("{}", pig_latin(&String::from("qhueen")));
    }
}
