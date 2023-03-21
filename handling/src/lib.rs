use std::fs::File;
use std::io::{Seek, Write};
use std::fs::OpenOptions;

pub fn open_or_create(file: &str, content: &str) {
    let mut new_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(file).unwrap_or_else(|_error| {
            File::create(file).unwrap()
        });

        write!(new_file, "{}", content).unwrap();
        new_file.rewind().unwrap();
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
