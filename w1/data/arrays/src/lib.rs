pub fn sum(a: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for i in 0..a.len() {
        result += a[i]
    }
    result
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [
        10, 10, 10, 10, 10,
        10, 10, 10, 10, 10,
        10, 10, 10, 10, 10,
        10, 10, 10, 10, 10,
        10, 10, 10, 10, 10,
        10, 10, 10, 10, 10,
        10, 10
    ]
}
