pub struct Message {
    content: String,
    user: String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    Message {
        content: ms,
        user: u,
    }
  }
  pub fn send_ms(&self) -> Option<&str> {
    match &self.content {
        content if content == "" || content.contains("stupid") => None,
        _ => Some(&self.content)
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        None => (false, "ERROR: illegal"),
        Some(content) => (true, content),
    }
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
