use nom::{
    branch::alt,
    character::{
        self,
        complete::{alpha1, multispace0, multispace1},
    },
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

use crate::expr::{BExpr, Fun, Inv, Var};

// Var <=> anyWord
// Inv <=> (expr1 expr2)
// Fun <=> (fn [x] x)
// (λx.y. ())

fn variable_name(input: &str) -> IResult<&str, &str> {
    alpha1.parse(input)
}

fn variable(input: &str) -> IResult<&str, BExpr> {
    variable_name.map(|name| Var(name)).parse(input)
}

fn invocation(input: &str) -> IResult<&str, BExpr> {
    delimited(
        character::complete::char('('),
        tuple((expression, preceded(multispace1, expression))),
        character::complete::char(')'),
    )
    .map(|(left, right)| Inv(left, right))
    .parse(input)
}

fn function(input: &str) -> IResult<&str, BExpr> {
    let variable_def = delimited(
        character::complete::char('\\'),
        variable_name,
        character::complete::char('.'),
    );
    delimited(
        character::complete::char('('),
        tuple((variable_def, preceded(multispace0, expression))),
        character::complete::char(')'),
    )
    .map(|(v, b)| Fun(v, b))
    .parse(input)
}

fn expression(input: &str) -> IResult<&str, BExpr> {
    alt((function, variable, invocation)).parse(input)
}

#[test]
fn test_parser() {
    assert_eq!(Ok(("", Var("foo"))), expression("foo"));
    assert_eq!(Ok(("", Fun("x", Var("x")))), expression("(\\x.x)"));
    assert_eq!(
        Ok(("", Fun("x", Inv(Var("x"), Var("x"))))),
        expression("(\\x.(x x))")
    )
}
