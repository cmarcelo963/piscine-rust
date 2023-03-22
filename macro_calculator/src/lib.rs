use json::{object};
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}
fn round_to_decimal(num: f64, decimal_places: i32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places);
    (num * multiplier).round() / multiplier
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let calories = foods.iter()
        .fold((0.0,0.0,0.0,0.0),| mut acc, food: &Food| {
            acc.0 += food.calories[1]
                .replace("kcal", "")
                .parse::<f64>().unwrap() * food.nbr_of_portions;
            acc.1 += food.carbs * food.nbr_of_portions;
            acc.2 += food.proteins * food.nbr_of_portions;
            acc.3 += food.fats *food.nbr_of_portions;
            println!("{:?}", acc);
            acc
            });
    let result = object!{
        cals: round_to_decimal(calories.0, 2),
       carbs: round_to_decimal(calories.1, 2),
        proteins: round_to_decimal(calories.2, 2),
        fats: round_to_decimal(calories.3, 2),
    };
    result
    
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
