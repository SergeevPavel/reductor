use nom::{
    branch::alt,
    character::{
        self,
        complete::{alpha1, multispace0, multispace1},
    },
    sequence::{delimited, preceded, tuple},
    IResult, Parser, combinator::{all_consuming, flat_map}, multi::{separated_list1, separated_list0, many_m_n, fold_many1}, error::ParseError,
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
    let invocation_body = flat_map(preceded(multispace0, expression), |func| {
        //todo aks why init is not FnOnce
        fold_many1(preceded(multispace1, expression), move || { func.clone() }, |func, arg| {
            Inv(func, arg)
        })
    });
    delimited(character::complete::char('('),
              invocation_body,
              character::complete::char(')')).parse(input)
}

fn function(input: &str) -> IResult<&str, BExpr> {
    let variables_def = delimited(character::complete::char('['),
                                  separated_list1(multispace1, variable_name),
                                  character::complete::char(']'));

    delimited(
        character::complete::char('('),
        tuple((variables_def, preceded(multispace0, expression))),
        character::complete::char(')'),
    )
    .map(|(vs, b)| vs.iter().rfold(b, |body, v| (Fun(v, body))))
    .parse(input)
}

fn expression(input: &str) -> IResult<&str, BExpr> {
    alt((function, variable, invocation)).parse(input)
}

pub fn parse_expression(input: &str) -> Option<BExpr> {
    Some(all_consuming(expression).parse(input).map(|(_, o)| o).unwrap())
}

#[test]
fn test_parser() {
    assert_eq!(Ok(("", Var("foo"))), variable("foo"));
    assert_eq!(Ok(("", Fun("x", Var("x")))), function("([x] x)"));
    assert_eq!(
        Some(Fun("x", Inv(Var("x"), Var("x")))),
        parse_expression("([x] (x x))")
    )
}
