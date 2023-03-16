pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original = c as f64;
    let exponential_function = original.exp();
    let natural_logarithm = original.abs().ln();
    (c, exponential_function, natural_logarithm)
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<&str> = a.split(" ").collect();
    let mut result = String::new();
    for number in numbers {
        let num: f64 = number.parse().unwrap();
        let exponential_string = num.exp().to_string();
        result.push_str(&exponential_string);
        result.push_str(" ");
    }
    result.pop();
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut new_vec: Vec<f64> = Vec::new();
    for val in &b {
        let natural_logarithm = (val.abs() as f64).ln();
        new_vec.push(natural_logarithm);
    }
    (b, new_vec)
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
