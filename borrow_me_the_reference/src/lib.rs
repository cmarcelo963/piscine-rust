pub fn delete_and_backspace(s: &mut String) {
    let mut new_string = String::new();
    let mut counter = 0;
    for character in s.chars() {
        if counter != 0 && character != '+' {
            counter -= 1;
            continue;
        }
        match character {
            '-' => {
                    new_string.pop();
            },
            '+' => {
                    counter += 1;
            },
            _ => {
                new_string.push(character);
            },
        }
    }
    s.clear();
    s.push_str(&new_string);
}


pub fn do_operations(v: &mut Vec<String>) {
    for s in v.iter_mut() {
        let parts: Vec<&str> = s.split(|c| c == '+' || c == '-').collect();
        let operators: Vec<char> = s.chars().filter(|c| *c == '+' || *c == '-').collect();
        let mut result = parts[0].parse::<i32>().unwrap();
        for i in 1..parts.len() {
            let operand = parts[i].parse::<i32>().unwrap();
            match operators[i-1] {
                '+' => result += operand,
                '-' => result -= operand,
                _ => (),
            }
        }
        *s = result.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
