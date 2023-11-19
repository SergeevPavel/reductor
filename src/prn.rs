use std::fmt::Display;

use crate::expr::{Expr, Var, Fun, Inv};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(name) => {
                f.write_str(name)?;
            },
            Expr::Inv(left, right) => {
                write!(f, "({} {})", left, right)?;
            },
            Expr::Fun(name, body) => {
                write!(f, "(\\{}.{})", name, body)?;
            },
        }
        Ok(())
    }
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Var("foo")), "foo");
    assert_eq!(format!("{}", Fun("x", Var("x"))), "(\\x.x)");
    assert_eq!(format!("{}", Fun("x", Inv(Var("x"), Var("x")))), "(\\x.(x x))");
}