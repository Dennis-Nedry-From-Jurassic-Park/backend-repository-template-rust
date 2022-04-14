pub fn render() {
    println!("module1");
}

#[derive(Debug)]
pub struct Subscriber(String);

impl Subscriber {
    pub fn parse(s: String) -> Result<Subscriber, String> {
        if s.trim().is_empty() {
            Err(format!("{} is not a valid subscriber.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Subscriber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Subscriber;
    use claim::assert_err;

    #[test]
    fn empty_string_is_rejected() {
        let empty = "".to_string();
        assert_err!(Subscriber::parse(empty));
    }
}