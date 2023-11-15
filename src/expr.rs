use crate::term::BTerm;


pub type Ident = String;

enum Expr {
    Var(Ident),
    Inv(Box<Expr>, Box<Expr>),
    Fun(Ident, Box<Expr>)
}

type BExpr = Box<Expr>;

//fn to_term(expr: &BExpr) -> BTerm {
//    
//}