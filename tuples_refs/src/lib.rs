#[derive(Debug, PartialEq, Eq)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    let result = student.0;
    return result
}

pub fn first_name(student: &Student) -> String {
    let result = student.1.clone();
    return result
}

pub fn last_name(student: &Student) -> String {
    let result = student.2.clone();
    return result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = (id(&student), first_name(&student),last_name(&student));
        assert_eq!(result, (20, "Pedro".to_string(), "Domingos".to_string()));
    }
}
