pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn diff(a: i32, b: i32) -> i32 {
    a - b
}
pub fn pro(a: i32, b: i32) -> i32 {
    a * b
}
pub fn quo(a: i32, b: i32) -> i32 {
    a / b
}
pub fn rem(a: i32, b: i32) -> i32 {
    a % b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let result2 = diff(-32768, 32767);
        assert_eq!(result2, -65535);
        let result3 = pro(-128, 127);
        assert_eq!(result3, -16256);
        let result4 = quo(59, 6);
        assert_eq!(result4, 9);
        let result5 = rem(59, 6);
        assert_eq!(result5, 5);
    }
}