use std::fs::File;

pub fn open_file(s: &str) -> File {
    let file_result = File::open(s);
    match file_result {
        Ok(file) => return file,
        Err(_error) => panic!("File not found"),
    };
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
