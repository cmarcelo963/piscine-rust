pub fn pig_latin(text: &str) -> String {
    println!("{}", text);
    fn is_vowel(c: char) -> bool {
        match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            }
    }
    let mut new_string = text.to_string();
    // let mut previous_char = '\0';
    let mut index = 0;
    loop {
        for char in text.chars() {
            if index == 0 && is_vowel(char){
                break;
            } else {
                index += 1;
            }
            // if previous_char == 'q' && char == 'u' {
            //     new_string.remove(0);
            //     new_string.push(char);
            //     previous_char = char;
            //     continue;
            // }
            if !is_vowel(char) {
                new_string.remove(0);
                new_string.push(char);
                previous_char = char;
                continue;
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
        println!("{}", pig_latin(&String::from("queen")));
    }
}
