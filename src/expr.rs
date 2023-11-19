use crate::term::{BTerm, Term, Lmb, Idx, App};


pub type Ident = String;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Var(Ident),
    Inv(Box<Expr>, Box<Expr>),
    Fun(Ident, Box<Expr>)
}

pub type BExpr = Box<Expr>;

#[allow(non_snake_case)]
pub fn Var<I: ToString>(i: I) -> BExpr {
    Box::new(Expr::Var(i.to_string()))
}

#[allow(non_snake_case)]
pub fn Inv(t1: BExpr, t2: BExpr) -> BExpr {
    Box::new(Expr::Inv(t1, t2))
}

#[allow(non_snake_case)]
pub fn Fun<I: ToString>(n: I, t: BExpr) -> BExpr {
    Box::new(Expr::Fun(n.to_string(), t))
}

fn idx_from_start(ctx: &Vec<Ident>, ident: &Ident) -> Option<i32> {
    ctx.iter().position(|x| *x == *ident).map(|idx| idx as i32)
}

fn idx_from_end(ctx: &Vec<Ident>, ident: &Ident) -> Option<i32> {
    ctx.iter().rev().position(|x| *x == *ident).map(|idx| idx as i32)
}

fn ident_from_end(ctx: &Vec<Ident>, idx: i32) -> Option<Ident> {
    ctx.iter().rev().nth(idx as usize).map(|s| s.to_owned())
}

fn to_term(expr: &BExpr) -> (BTerm, Vec<Ident>) {
    let mut free = Vec::new();
    let mut bounded = Vec::new();
    fn go(free: &mut Vec<Ident>, bounded: &mut Vec<Ident>, expr: &Expr) -> BTerm {
        match expr {
            Expr::Fun(v, body) => {
                bounded.push(v.clone());
                let bterm = go(free, bounded, body);
                Lmb(bounded.pop().unwrap(), bterm)
            },
            Expr::Var(v) => {
                match idx_from_end(bounded, v) {
                    Some(idx) =>  {
                        Idx(idx)
                    }
                    None => {
                        match idx_from_start(free, v) {
                            Some(idx) => {
                                Idx(bounded.len() as i32 + idx)
                            },
                            None => {
                                free.push(v.clone());
                                Idx(bounded.len() as i32 + free.len() as i32 - 1)
                            },
                        }
                    }
                }
            },
            Expr::Inv(left, right) => {
                App(go(free, bounded, left), go(free, bounded, right))
            },
        }
    }
    (go(&mut free, &mut bounded, expr), free)
}

fn make_fresh(ctx: &Vec<Ident>, ident: &Ident) -> Ident {
    let mut ident = ident.clone();
    loop {
        if !ctx.contains(&ident) {
            break ident;
        }
        let parts: Vec<&str> = ident.split("_").collect();
        match &parts[..] {
            [var_name] => {
                ident = format!("{}_1", var_name).to_string();
            }
            [var_name, suffix] => {
                let next_suffix = suffix.parse::<i32>().unwrap();
                ident = format!("{}_{}", var_name, next_suffix + 1);
            }
            _ => {}
        }
    }
}

fn to_expr(free: Vec<Ident>, term: &BTerm) -> BExpr {
    let mut ctx = free.clone();
    ctx.reverse();
    fn go(ctx: &mut Vec<Ident>, term: &Term) -> BExpr {
        match term {
            Term::Lmb(ident, body) => {
                let fresh_ident = make_fresh(&ctx, ident);
                ctx.push(fresh_ident);
                let body = go(ctx, body);
                Fun(ctx.pop().unwrap(), body)
            },
            Term::Idx(idx) => {
                let ident = ident_from_end(ctx, *idx)
                    .unwrap_or_else(|| panic!("No {:?} in {:?}", idx, ctx));
                Var(ident)
            },
            Term::App(left, right) => {
                Inv(go(ctx, left), go(ctx, right))
            },
        }
    }
    go(&mut ctx, term)
}

#[test]
fn test_to_term() {
    // GHCi> one = Lam "s" $ Lam "z" $ Var "s" :@ Var "z"
    // GHCi> e2t one
    // ([],Lmb "s" (Lmb "z" (Idx 1 :@: Idx 0)))
    // GHCi> tst1 = Lam "x" $ Var "x" :@ Var "y" :@ Var "z"
    // GHCi> e2t tst1
    // (["y","z"],Lmb "x" ((Idx 0 :@: Idx 1) :@: Idx 2))
    let expr1 = Fun("s", Fun("z", Inv(Var("s"), Var("z"))));
    let term1 = Lmb("s", Lmb("z", App(Idx(1), Idx(0))));
    assert_eq!((term1, vec![]), to_term(&expr1));
    let expr2 = Fun("x", Inv(Var("w"), Inv(Inv(Var("x"), Var("y")), Var("w"))));
    let term2 = Lmb("x", App(Idx(1), App(App(Idx(0), Idx(2)), Idx(1))));
    assert_eq!((term2, vec!["w".to_string(), "y".to_string()]), to_term(&expr2));
}

#[test]
fn test_to_expr() {
    let expr1 = Fun("s", Fun("z", Inv(Var("s"), Var("z"))));
    let term1 = Lmb("s", Lmb("z", App(Idx(1), Idx(0))));
    assert_eq!(expr1, to_expr(vec![], &term1));
    let expr2 = Fun("x", Inv(Var("w"), Inv(Inv(Var("x"), Var("y")), Var("w"))));
    let term2 = Lmb("x", App(Idx(1), App(App(Idx(0), Idx(2)), Idx(1))));
    assert_eq!(expr2, to_expr(vec!["w".to_string(), "y".to_string()], &term2))
}

#[test]
fn test_freshnes() {
    let t = Lmb("z", Lmb("z", Lmb("z", App(Idx(0), App(Idx(1), Idx(2))))));
    let e = Fun("z", Fun("z_1", Fun("z_2", Inv(Var("z_2"), Inv(Var("z_1"), Var("z"))))));
    assert_eq!(e, to_expr(vec![], &t));
}