pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match &server {
        Ok(url) => match security_level {
            Security::BlockServer => server.unwrap_err(),
            _ => url.to_string(),
        }
        Err(_) => match security_level {
            Security::Unknown => server.unwrap(),
            Security::High => panic!("ERROR: program stops"),
            Security::Medium => return "WARNING: check the server".to_string(),
            Security::Low => return match &server {
                Ok(url) => "Not found: ".to_string() + &url.to_string(),
                Err(error) => "Not found: ".to_string() + &error.to_string(),
            },
            Security::BlockServer => server.unwrap_err(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("{}", fetch_data(Ok("server1.com".to_string()), Security::Medium));
        // println!("{}", fetch_data(Err(String::new()), Security::Medium));
        // println!("{}", fetch_data(Err("server2.com".to_string()), Security::Low));
        fetch_data(Ok("server1.com".to_string()), Security::Medium);
    }
}
