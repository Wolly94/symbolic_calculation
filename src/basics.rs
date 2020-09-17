pub const OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];
const LOPERATORS: [char; 3] = ['+', '*', '^'];
const ROPERATORS: [char; 2] = ['-', '/'];
pub const FUNCTIONS: [&str; 8] = ["sinh", "cosh", "tanh", "sin", "cos", "tan", "exp", "ln"];

#[derive(Debug)]
pub enum StringParseError {
    ErrorMessage(String),
}

impl std::fmt::Display for StringParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for StringParseError {
    fn description(&self) -> &str {
        match self {
            StringParseError::ErrorMessage(s) => s
        }
    }
}

pub fn remove_braces(s: &str) -> &str {
    if s.len() > 1 {
        if (s.chars().next().unwrap() == '(') && (s.chars().last().unwrap() == ')') {
            let mut l = 0;
            for d in s[..s.len()-1].chars() {
                if d == '(' {
                    l = l+1;
                } else if d == ')' {
                    l = l-1;
                } else if l == 0 {
                    return s;
                }
            }
            if l == 1 {
                return remove_braces(&s[1..s.len()-1])
            } else {
                // println!("Incorrect braces");
                return s;
            }
        }
    }
    return s;
}

pub fn combine(s: &str, c: Option<usize>, t: Option<&str>) -> String {
    return match (c, t) {
        (None, Some(t)) => t.to_string()+"("+remove_braces(s)+")",
        (Some(i), Some(t)) => {
            let c = OPERATORS[i];
            let v1 = split(s);
            let v2 = split(t);
            let a: String = match v1.1 {
                Some(i1) => {
                    let c1 = OPERATORS[i1];
                    if (['*', '/'].contains(&c) && ['+', '-'].contains(&c1)) ||
                        (c == '^' && ['+', '-', '*', '/', '^'].contains(&c1)) {
                        "(".to_string()+s+&")"
                    } else {
                        s.to_string()
                    }
                }
                _ => s.to_string()
            };
            let b: String = match v2.1 {
                Some(i2) => {
                    let c2 = OPERATORS[i2];
                    if (c == '-' && c2 == '-') ||
                        (c == '*' && ['+', '-'].contains(&c2)) ||
                        (c == '/' && ['+', '-', '*', '/'].contains(&c2)) ||
                        (c == '^') {
                            "(".to_string()+t+&")"
                        } else {
                            t.to_string()
                        }
                }
                None => t.to_string()
            };
            a+&c.to_string()+&b
        }
        _ => s.to_string(),
    }
}

pub fn split(s: &str) -> (&str, Option<usize>, Option<&str>) {
    for (pos, c) in OPERATORS.iter().enumerate() {
        if LOPERATORS.contains(c) {
            let mut l = 0;
            for (i, d) in s.chars().enumerate() {
                if d == '(' {
                    l = l+1
                } else if d == ')' {
                    l = l-1
                } else if (l == 0) && (d == *c) {
                    return (&s[..i], Some(pos), Some(&s[i+1..]))
                }
            }
        }
        if ROPERATORS.contains(c) {
            let mut l = 0;
            if *c == '/' {
                for (i, d) in s.chars().rev().enumerate() {
                    if d == '(' {
                        l = l+1
                    } else if d == ')' {
                        l = l-1
                    } else if (l == 0) && (d == *c) {
                        let j = s.len()-i-1;
                        return (&s[..j], Some(pos), Some(&s[j+1..]))
                    }
                }
            }
            else if *c == '-' {
                let mut old_c = s.chars().rev().next().unwrap();
                for (i, d) in s[1..s.len()].chars().rev().enumerate() {
                    if (l == 0) && (((old_c == *c) && !(['+', '*', '/', '^'].contains(&d))) || d == '-') {
                        let j = s.len()-i-1;
                        return (&s[..j], Some(pos), Some(&s[j+1..]))
                    } else if d == '(' {
                        l = l+1
                    } else if d == ')' {
                        l = l-1
                    }
                    old_c = d;
                }
                if (*c == '-') && (s.chars().next().unwrap() == '-') && (s != "-1") {
                    return ("-1", Some(2), Some(&s[1..]))
                }
            }
        }
    }
    for (_pos, f) in FUNCTIONS.iter().enumerate() {
        if f.len() <= s.len() {
            if *f == &s[..f.len()] {
                return (&s[f.len()..], None, Some(f))
            }
        }
    }
    return (s, None, None)
}

pub fn check(s: &str) -> Result<Option<i32>, StringParseError> {// assuming that split(s) returns (s, None, None)
    let i = s.to_string().parse::<i32>();
    match i {
        Ok(integer) => return Ok(Some(integer)),
        Err(_) => {
            if s.len() == 0 {
                return Err(StringParseError::ErrorMessage("Invalid string: ".to_string() + s))
            }
            for c in 0..9 {
                if c.to_string() == s.chars().next().unwrap().to_string() {
                    return Err(StringParseError::ErrorMessage("Invalid string: ".to_string() + s))
                }
            }
            for c in ["*", "/", "^"].iter() {
                if c == &&s.chars().next().unwrap().to_string() {
                    return Err(StringParseError::ErrorMessage("Invalid string: ".to_string() + s))
                }
            }
            let forbidden_char = ["(", ")", "{", "}"];
            for c in &forbidden_char {
                if s.to_string().contains(c) {
                    return Err(StringParseError::ErrorMessage("Invalid string: ".to_string() + s))
                }
            };
            for c in s.chars() {
                if c == ' ' {
                    return Err(StringParseError::ErrorMessage("Invalid string: ".to_string() + s))
                }
            }
            return Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_braces() {
        let i = ["((4+3))", "()", "", "(4+3)^3+(4+3)"];
        let o = ["4+3", "", "", "(4+3)^3+(4+3)"];
        for j in 0..i.len() {
            assert_eq!(remove_braces(i[j]), o[j]); 
        }
    }

    #[test]
    fn test_split() {
        let i = ["(4+3)*2", "4+3^2", "4/(3*2)", "sin(5*x)", "-x", "3-2"];
        let o = [("(4+3)", Some(2), Some("2")), ("4", Some(0), Some("3^2")), ("4", Some(3), Some("(3*2)")), ("(5*x)", None, Some("sin")), ("-1", Some(2), Some("x")), ("3", Some(1), Some("2"))];
        for j in 0..i.len() {
            assert_eq!(split(i[j]), o[j]); 
        }
    }

    #[test]
    fn test_combine() {
        let i = [("4+3^2", None, Some("sin")), ("4+3", Some(4), Some("2*sin(4)")), ("4+1", Some(3), Some("3/3"))];
        let o = ["sin(4+3^2)", "(4+3)^(2*sin(4))", "(4+1)/(3/3)"];
        for j in 0..i.len() {
            assert_eq!(combine(i[j].0, i[j].1, i[j].2), o[j]); 
        }
    }
}
