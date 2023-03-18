pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result: String = String::new();
    let mut is_space: bool = false;
    for (i, c) in input.char_indices() {
        if i == 0 {
            result += &c.to_uppercase().collect::<String>() 
        } else if is_space {
            result += &c.to_uppercase().collect::<String>();
            is_space = false
        } else {
            result += &c.to_string();
            if c == ' ' {
                is_space = true
            }
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result: String = String::new();
    for c in input.chars() {
        if c == ' ' {
            result += &c.to_string();

        } else if c.is_ascii_uppercase() {
            result += &c.to_lowercase().collect::<String>()
        } else if c.is_ascii_lowercase() {
            result += &c.to_uppercase().collect::<String>() 
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
        println!("{}",change_case("heLLo THere"));
    }
}
