#[derive(Debug, Clone, Hash)]
pub struct Var(pub String);

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Var(s) = self;
        return write!(f, "{}", s)
    }
}

impl std::cmp::PartialEq for Var {
    fn eq(&self, other: &Var) -> bool {
        let Var(s1) = self;
        let Var(s2) = other;
        s1 == s2
    }
}

impl std::cmp::Eq for Var {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_and_eq() {
        let x = Var("x".to_string());
        let y = Var("y".to_string());
        let z = Var("x".to_string());
        assert_eq!(x, z);
        assert_eq!("x", x.to_string());
        assert_eq!("y", y.to_string());
    }
}
