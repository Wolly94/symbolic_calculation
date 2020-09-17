use super::basics;
use super::var::*;
pub trait Power { fn pow(a: Self, b: Self) -> Self;
}

#[derive(Debug, Clone, Hash)]
pub enum Term {
    N(i32),
    V(Var),
    //S(Box<Term>),
    //B(Box<Term>)
    FT(String, Box<Term>),
    ST(Vec<Term>),
    PT(Vec<Term>),
    //DT(Box<(Term, Term)>),
    ET(Box<(Term, Term)>),
    INVALID(String),
}

impl Term {
    pub fn add_raw(self, other: Self) -> Self {
        return match (self, other) {
            (Term::INVALID(s), _) | (_, Term::INVALID(s)) => Term::INVALID(s),
            (Term::ST(mut v1), Term::ST(mut v2)) => {v1.append(&mut v2); Term::ST(v1)},
            (Term::ST(mut v), t) => {v.push(t); Term::ST(v)},
            (t, Term::ST(mut v)) => {let mut v2 = vec![t]; v2.append(&mut v); Term::ST(v2)},
            (a, b) => Term::ST(vec![a, b]),
        }
    }

    pub fn mul_raw(self, other: Self) -> Self {
        return match (self, other) {
            (Term::INVALID(s), _) | (_, Term::INVALID(s)) => Term::INVALID(s),
            (Term::PT(mut v1), Term::PT(mut v2)) => {v1.append(&mut v2); Term::PT(v1)},
            (Term::PT(mut v), t) => {v.push(t); Term::PT(v)},
            (t, Term::PT(mut v)) => {let mut v2 = vec![t]; v2.append(&mut v); Term::PT(v2)},
            (a, b) => Term::PT(vec![a, b]),
        }
    }

    pub fn pow_raw(self, other: Self) -> Self {
        return match (self, other) {
            (Term::INVALID(s), _) | (_, Term::INVALID(s)) => Term::INVALID(s),
            (a, b) => Term::ET(Box::new((a, b))),
        }
    }

    pub fn pow(self, other: Self) -> Self {
        match (self, other) {
            (Term::INVALID(s), _) | (_, Term::INVALID(s)) => Term::INVALID(s),
            (Term::N(n1), Term::N(n2)) => {
                if (n1 == 0) && (n2 <= 0) {
                    Term::INVALID("0^0".to_string())
                } else if n2 < 0 {
                    Term::ET(Box::new((Term::N(n1).pow(Term::N(-n2)), Term::N(-1))))
                } else {
                    // TODO: catch int overflow ....
                    Term::N(n1.pow(n2 as u32))
                }
            },
            (Term::N(n), t) => if n == 0 {
                    Term::N(0)
                } else if n < 0 {
                    Term::ET(Box::new((Term::N(n), t.clone())))
                } else {
                    Term::ET(Box::new((Term::N(n), t.clone())))
                },
            (t, Term::N(n)) => {
                if n == 0 {
                    Term::N(1)
                } else if n == 1 {
                    t
                } else if n > 0 {
                    Term::ET(Box::new((t, Term::N(n))))
                } else {
                    Term::ET(Box::new((t.clone(), Term::N(n))))
                }
            },
            (Term::ET(b), t) => {
                Term::ET(Box::new(((*b).0, (*b).1*t.clone())))
            },
            (t1, t2) => {
                Term::ET(Box::new((t1.clone(), t2.clone())))
            },
        }
    }

    fn base(&self) -> &Self {
        return match self {
            Term::ET(bb) => &(*bb).0,
            _ => &self,
        }
    }

    fn expo(&self) -> &Self {
        return match self {
            Term::ET(bb) => &(*bb).1,
            _ => &Term::N(1),
        }
    }
}

impl Power for Term {
    fn pow(a: Self, b: Self) -> Self {
        return match (a, b) {
            (Term::INVALID(s), _) | (_, Term::INVALID(s)) => Term::INVALID(s),
            (Term::N(0), Term::N(0)) => Term::INVALID("0^0".to_string()),
            (Term::N(n), _) if (n == 0) || (n == 1) => Term::N(n),
            (_, Term::N(0)) => Term::N(1),
            (a, Term::N(1)) => a,
            (Term::ET(b), t) => Term::ET(Box::new(((*b).0, (*b).1*t.clone()))),
            (t1, t2) => t1.pow_raw(t2),
        }
    }
}

impl std::ops::Add<Term> for Term {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return match (self, other) {
            (Term::N(0), t) | (t, Term::N(0)) => t,
            (a, b) => a.add_raw(b)
        }
    }
}

impl std::ops::Sub for Term {
    type Output = Term;

    fn sub(self, other: Self) -> Self {
        return self + (-other)
    }
}

impl std::ops::Mul for Term {
    type Output = Term;

    fn mul(self, other: Term) -> Term {
        return match (self, other) {
            (Term::N(1), t) | (t, Term::N(1)) => t,
            (Term::N(0), _) | (_, Term::N(0)) => Term::N(0),
            (a, b) => a.mul_raw(b),
        }
    }
}

impl std::ops::Div for Term {
    type Output = Term;

    fn div(self, other: Term) -> Term {
        return self * other.pow(Term::N(-1));
    }
}

impl std::ops::Neg for Term {
    type Output = Term;

    fn neg(self) -> Term {
        return Term::N(-1)*self
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            Term::INVALID(s) => write!(f, "Invalid Term: {}", s),
            Term::N(n) => write!(f, "{}", n),
            Term::V(v) => write!(f, "{}", v),
            Term::FT(uf, b) => write!(f, "{}({})", uf, b),
            Term::ST(l) => {
                let mut r = l[0].to_string();
                for t in &l[1..] {
                    r = basics::combine(&r, Some(0), Some(&t.to_string()));
                }
                if (r.len() >= 2) && (&r[..2] == "0+") {
                    r = r[2..].to_string()
                }
                write!(f, "{}", r)
                },
            Term::PT(l) => {
                let mut r = l[0].to_string();
                for t in &l[1..] {
                    r = basics::combine(&r, Some(2), Some(&t.to_string()));
                }
                if (r.len() >= 2) && (&r[..2] == "1*") {
                    r = r[2..].to_string()
                } else if (r.len() >= 3) && (&r[..3] == "-1*") {
                    r = "-".to_string()+&r[3..]
                }
                write!(f, "{}", r)
                },
            Term::ET(b) => {
                    write!(f, "{}", basics::combine(&(b.0).to_string(), Some(4), Some(&(b.1).to_string())))
                }
        }
    }
}

impl std::cmp::Eq for Term {}

impl std::cmp::PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        return match (self, other) {
            (Term::INVALID(_), Term::INVALID(_)) => true,
            (Term::N(n1), Term::N(n2)) => n1 == n2,
            (Term::V(var1), Term::V(var2)) => var1 == var2,
            (Term::FT(uf1, t1), Term::FT(uf2, t2)) => (uf1 == uf2) && (*t1) == (*t2),
            (Term::ST(l1), Term::ST(l2)) | (Term::PT(l1), Term::PT(l2)) => { // not working properly, e.g. l1 = [a, b, b, b] and l2 = [a, a, b, b] !!!
                let mut r = l1.len() == l2.len();
                if r {
                    let mut r2 = false;
                    for t1 in l1.iter() {
                        for t2 in l2.iter() {
                            if !r2 {
                                if t1 == t2 {
                                    r2 = true;
                                    break;
                                };
                            };
                        };
                        if !r2 {
                            r = false;
                            break;
                        };
                    };
                    if r {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            (Term::ET(bb1), Term::ET(bb2)) => ((*bb1).0 == (*bb2).0) && ((*bb1).1 == (*bb2).1),
            _ => false,
        }
    }
}

impl From<i32> for Term {
    fn from(i: i32) -> Self {
        Term::N(i)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        let v = basics::split(basics::remove_braces(s));    
        return match (v.1, v.2) {
            (None, Some(t)) => Term::FT(t.to_string(), Box::new(Term::from(v.0))),
            (Some(i), Some(t)) => {
                let t1 = Term::from(basics::remove_braces(v.0));
                let t2 = Term::from(basics::remove_braces(t));
                match basics::OPERATORS[i] {
                    '+' => t1+t2,
                    '-' => t1-t2,
                    '*' => t1*t2,
                    '/' => t1/t2,
                    '^' => t1.pow(t2),
                    c => Term::INVALID("operator not implemented: ".to_string() + &c.to_string()),
                }
                },
            (_, None) => match basics::check(v.0) {
                    Ok(None) => Term::V(Var(v.0.to_string())),
                    Ok(Some(i)) => Term::N(i),
                    Err(e) => Term::INVALID(e.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_display() {
        let z = Var("z".to_string());
        let i = [Term::N(5), Term::V(z.clone()), Term::FT("sin".to_string(), Box::new(Term::N(-4))), Term::ET(Box::new((Term::ST(vec![Term::N(1), Term::V(z.clone())]), Term::PT(vec![Term::N(4), Term::N(7)])))), Term::INVALID("Hello there".to_string())];
        let o = ["5", "z", "sin(-4)", "(1+z)^(4*7)", "Invalid Term: Hello there"];
        for j in 0..i.len() {
            assert_eq!(i[j].to_string(), o[j]); 
        }
    }

    #[test]
    fn test_term_equality() {
        let v1 = Var("x".to_string());
        let v2 = Var("y".to_string());
        let t1 = Term::ST(vec![Term::PT(vec![Term::V(v1.clone()), Term::V(v2.clone())]), Term::N(4)]);
        let t2 = Term::ST(vec![Term::N(4), Term::PT(vec![Term::V(v2.clone()), Term::V(v1.clone())])]);
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_parse_string() {
        let z = Var("z".to_string());
        let i = [Term::N(5), Term::V(z.clone()),
                Term::FT("sin".to_string(), Box::new(Term::PT(vec![Term::N(-1), Term::N(4)]))), Term::ET(Box::new((Term::ST(vec![Term::N(1), Term::V(z.clone())]), Term::PT(vec![Term::V(z.clone()), Term::N(7)])))), Term::INVALID("Hello there".to_string())];
        let o = ["5", "z", "sin(-4)", "(1+z)^(z*7)", "In skjdh "];
        for j in 0..i.len() {
            assert_eq!(i[j], Term::from(o[j])); 
        }
    }

    // wanted behaviour not implemented yet
    //#[test]
    fn test_term_operations() {
        let i = ["5+(4*3)", "z", "sin(-4)", "(1+x)^(4*7)", "(x", "4*x+3*x", "x^z*x"];
        let o = ["17", "z", "sin(-4)", "(3-2+x)^28", "Invalid Term", "7*x", "x^(z+1)"];
        for j in 0..i.len() {
            assert_eq!(Term::from(i[j]), Term::from(o[j]));
        }
    }
}
