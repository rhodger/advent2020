use regex::Regex;

pub struct Password {
    rule: (char, u8, u8),
    pass: String,
}

impl Password {
    pub fn new(line: &str) -> Result<Self, String> {
        let strings = line.split(": ").collect::<Vec::<&str>>();
        let (target, lower, upper) = match parse_rule(strings[0]) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        Ok(Password {
            rule: (target, lower, upper),
            pass: strings[1].to_string(),
        })
    }

    pub fn test_validity(&self) -> bool {
        let relogic = format!("{}", self.rule.0);
        let re = match Regex::new(&relogic) {
            Ok(x) => x,
            _ => return false,
        };
        let matches = re.find_iter(&self.pass);
        let num_matches = matches.count() as u8;
        if num_matches >= self.rule.1 && num_matches <= self.rule.2 {
            return true;
        } else {
            return false;
        }
    }

    pub fn test_alternate_validity(&self) -> bool {
        let (i, j) = (self.rule.1 as usize - 1, self.rule.2 as usize - 1);
        let target_char = self.rule.0;
        let indexable_pass = self.pass.as_bytes();
        if indexable_pass[i] as char == target_char || indexable_pass[j] as char == target_char {
            return true;
        } else {
            return false;
        }
    }
}

fn parse_rule(x: &str) -> Result<(char, u8, u8), String> {
    let strings = x.split(' ').collect::<Vec::<&str>>();
    if strings.len() != 2 {
        return Err("Wrong number of args in rule".to_string());
    }
    let limits = strings[0].split('-').collect::<Vec::<&str>>();
    if limits.len() != 2 {
        return Err("Wrong number of limits found in rule".to_string());
    }
    let lower: u8 = match limits[0].parse() {
        Ok(x) => x,
        _ => return Err("Invalid lower limit".to_string()),
    };
    let upper: u8 = match limits[1].parse() {
        Ok(x) => x,
        _ => return Err("Invalid upper limit".to_string()),
    };
    let target_char: char = match strings[1].chars().next() {
        Some(x) => x,
        _ => return Err("Invalid target character".to_string()),
    };

    Ok((target_char, lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let out = parse_rule("1-3 f");
        if out.is_err() {
            println!("Error: {:?}", out);
        }
        let out = out.unwrap();
        assert_eq!(out.0, 'f');
        assert_eq!(out.1, 1);
        assert_eq!(out.2, 3);
    }

    #[test]
    fn test_password_constructor() {
        let password_res = Password::new("1-3 f: test");
        let password = password_res.unwrap();
        assert_eq!(('f', 1, 3), password.rule);
        assert_eq!("test", password.pass);
    }

    #[test]
    fn test_test_validity() {
        let password_1 = Password::new("1-3 b: burger").unwrap();
        let password_2 = Password::new("1-3 b: nowhere").unwrap();
        let password_3 = Password::new("3-4 g: burger").unwrap();

        assert_eq!(password_1.test_validity(), true);
        assert_eq!(password_2.test_validity(), false);
        assert_eq!(password_3.test_validity(), false);
    }

    #[test]
    fn test_test_alternate_validity() {
        let password_1 = Password::new("1-3 b: burger").unwrap();
        let password_2 = Password::new("1-3 b: nowhere").unwrap();
        let password_3 = Password::new("3-4 g: burger").unwrap();

        assert_eq!(password_1.test_alternate_validity(), true);
        assert_eq!(password_2.test_alternate_validity(), false);
        assert_eq!(password_3.test_alternate_validity(), true);
    }
}