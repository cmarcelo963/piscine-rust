use std::collections::HashMap;
use std::num::ParseFloatError;

struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    fn opt_flag(short_hand: &str, desc: &str) -> Flag {
        let long_hand = format!("--{}", short_hand);
        let short_hand = format!("-{}", short_hand.chars().next().unwrap());
        Flag { short_hand, long_hand, desc: desc.to_owned() }
    }
}

type Callback = fn(f64, f64) -> Result<f64, ParseFloatError>;

struct FlagsHandler {
    flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    fn add_flag(&mut self, flag: Flag, callback: Callback) {
        let key = (flag.short_hand.clone(), flag.long_hand.clone());
        self.flags.insert(key, callback);
    }

    fn exec_func(&self, short_hand: &str, long_hand: &str, args: &[String]) -> Result<String, String> {
        if let Some(callback) = self.flags.get(&(short_hand.to_owned(), long_hand.to_owned())) {
            if args.len() != 2 {
                return Err("Invalid number of arguments".to_owned());
            }
            let a = args[0].parse::<f64>();
            let b = args[1].parse::<f64>();
            match (a, b) {
                (Ok(a), Ok(b)) => Ok(callback(a, b).to_string()),
                _ => Err("Invalid arguments".to_owned()),
            }
        } else {
            Err("Flag not found".to_owned())
        }
    }
}

fn div(a: f64, b: f64) -> Result<f64, ParseFloatError> {
    Ok(a / b)
}

fn rem(a: f64, b: f64) -> Result<f64, ParseFloatError> {
    Ok(a % b)
}