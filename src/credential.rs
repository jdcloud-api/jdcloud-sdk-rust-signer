pub struct Credential {
    ak: String,
    sk: String
}

impl Credential {
    pub fn new(ak: String, sk: String) -> Credential {
        Credential {
            ak,
            sk
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.ak.is_empty() && !self.sk.is_empty()
    }

    pub fn ak(&self) -> &str {
        &self.ak
    }

    pub fn sk(&self) -> &str {
        &self.sk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ak = "1";
        let sk = "2";
        let _c = Credential::new(ak.to_string(), sk.to_string());
    }

    #[test]
    fn test_is_valid() {
        assert!(Credential::new("a".to_string(), "b".to_string()).is_valid());
        assert!(!Credential::new("a".to_string(), "".to_string()).is_valid());
        assert!(!Credential::new("".to_string(), "b".to_string()).is_valid());
        assert!(!Credential::new("".to_string(), "".to_string()).is_valid());
    }
}
