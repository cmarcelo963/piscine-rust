pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let split_str = s.split_whitespace();
    let mut vec: Vec<u32> = Vec::new();
    for strs in split_str {
        if strs.ends_with('k') {
            let mut num = strs[..strs.len() - 1].parse::<f64>().unwrap();
            // let num = strs.parse::<f64>().unwrap() ;
            println!("{num} this is number with k.");
            num = num * 1000.0;
            println!("{num} it looks like this now ");
            vec.push(num as u32);
        } else {
            vec.push(strs.parse::<u32>().unwrap())
        }
    }
    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");

        // creating a variable and we save it in the Heap
        let a_h = transform_and_save_on_heap(new_str);
        println!("{}", a_h);
        // println!("Box value : {:?}", &a_h);
        // println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));

        // let a_b_v = take_value_ownership(a_h);
        // println!("value : {:?}", &a_b_v);
        // println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
        // // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed

    }
}
