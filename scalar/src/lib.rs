pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}
pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}
pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}
pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}
pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(0, 255);
        assert_eq!(result, 255);
        let result2 = diff(3, 2);
        assert_eq!(result2, 1);
        let result3 = pro(2,4);
        assert_eq!(result3, 8);
        let result4 = quo(-128.23, 2.0);
        assert_eq!(result4, -64.115);
        let result5 = rem(-128.23, 2.0);
        assert_eq!(result5, -0.22999573);
    }
}