mod term;
mod expr;
mod pars;
mod prn;

use std::{fmt::Debug, io::Write, path::Path};

use expr::{to_term, to_expr};
use pars::parse_expression;
use term::*;

fn repl() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    loop {
        stdin.read_line(&mut buffer).unwrap();
        eprintln!("{:?}", buffer.trim());
        let expr = parse_expression(&buffer.trim()).unwrap();
        let (term, ctx) = to_term(&expr);
        eprintln!("term: {:?} ctx: {:?}", term, ctx);
        let norm_term = normal_form(step_norm, term);
        eprintln!("norm_term: {:?}", norm_term);
        let norm_expr = to_expr(ctx, &norm_term);
        eprintln!("norm_expr: {:?}", norm_expr);
        println!("{}", norm_expr);
        //        write!(stdout, "{}", norm_expr).unwrap();
    }
}

fn exec<P: AsRef<Path>>(path: P) {
    let input = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("No such file: {:?}", path.as_ref()));
    let expr = parse_expression(input.trim()).unwrap();
    let (term, ctx) = to_term(&expr);
    eprintln!("term: {:?} ctx: {:?}", term, ctx);
    let norm_term = normal_form(step_norm, term);
    eprintln!("norm_term: {:?}", norm_term);
    let norm_expr = to_expr(ctx, &norm_term);
    eprintln!("norm_expr: {:?}", norm_expr);
    println!("{}", norm_expr);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    exec(&args[1])
}