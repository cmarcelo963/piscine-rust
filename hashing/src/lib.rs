use std::collections::HashMap;
pub fn bigger(h: HashMap<&i32, i32>) -> i32 {
    let mut highest_value: i32 = 0;
    let mut highest_key:i32 = 0;
    for (key, value) in h.iter() {
        if value > &highest_value {
            highest_value = *value;
            highest_key = **key
        }
    }
    highest_key
}
pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let mean: f64 = sum as f64 / list.len() as f64;
    return mean
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted = list.clone();
    sorted.sort();
    println!("Vec {:?}", sorted);
    let mid_index: usize = sorted.len() / 2;
    let is_odd: bool = if sorted.len() % 2 == 0 {false} else {true};
    if is_odd {
        return sorted[mid_index]
    } else {
        let mid2_index: usize = mid_index - 1;
        let result = sorted[mid_index] + sorted[mid2_index];
        println!("1st: {}, 2nd: {}", sorted[mid_index], sorted[mid2_index]);
        return result / 2
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut list_map = HashMap::new();
    for item in list {
        *list_map.entry(item).or_insert(0) += 1;
    }
    return bigger(list_map)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    let v = vec![2, 1, 5, 2, 7, 4];
        println!("mean {}", mean(&v));
        println!("median {}", median(&v));
        println!("mode {}", mode(&v));
    }
}
