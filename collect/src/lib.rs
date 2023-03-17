pub fn bubble_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let n = vec.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ref mut v = vec![3, 2, 4, 5, 1, 7];
        let mut b = v.clone();
        bubble_sort(v);
        println!("{:?}", v);

        b.sort();
        println!("{:?}", b);
    }
}
