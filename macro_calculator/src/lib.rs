use json::{object};
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let calories = foods.iter()
        .fold((0.0,0.0,0.0,0.0),| mut acc, food: &Food| {
            acc.0 += ((food.calories[1]
                .replace("kcal", "")
                .parse::<f64>().unwrap() * food.nbr_of_portions) * 100.).round() / 100.;
            acc.1 += ((food.carbs * food.nbr_of_portions) * 100.).round()/ 100.;
            acc.2 += ((food.proteins * food.nbr_of_portions) * 100.).round() / 100.;
            acc.3 += ((food.fats *food.nbr_of_portions) * 100.).round() / 100.;
            println!("{:?}", acc);
            acc
            });
    let result = object!{
        cals: calories.0,
       carbs: calories.1,
        proteins: calories.2 ,
        fats: calories.3,
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
