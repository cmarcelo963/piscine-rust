pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (index, element) in array.iter().enumerate() {
        if element == &key {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        let f = search(&ar, 6);
        println!(
            "the element 6 is in the position {:?} in the array {:?}",
            f, ar
        );
    }
}
