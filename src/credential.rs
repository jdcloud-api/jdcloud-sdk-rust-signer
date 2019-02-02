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
}
