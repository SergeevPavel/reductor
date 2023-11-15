use crate::expr::Ident;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Term {
    Idx(i32),
    App(Box<Term>, Box<Term>),
    Lmb(Ident, Box<Term>),
}

pub type BTerm = Box<Term>;

#[allow(non_snake_case)]
pub fn Idx(i: i32) -> BTerm {
    Box::new(Term::Idx(i))
}

#[allow(non_snake_case)]
pub fn App(t1: BTerm, t2: BTerm) -> BTerm {
    Box::new(Term::App(t1, t2))
}

#[allow(non_snake_case)]
pub fn Lmb<I: ToString>(n: I, t: BTerm) -> BTerm {
    Box::new(Term::Lmb(n.to_string(), t))
}

fn shift_ref(v: i32, t: &mut Term) {
    fn go(v: i32, depth: i32, t: &mut Term) {
        match t {
            Term::Idx(i) => {
                if *i >= depth {
                    *i = *i + v;
                }
            },
            Term::App(t1, t2) => {
                go(v, depth, t1);
                go(v, depth, t2);

            },
            Term::Lmb(_, t1) => {
                go(v, depth + 1, t1)
            },
        }
    }
    go(v, 0, t);
}

fn shift(v: i32, mut t: BTerm) -> BTerm {
    shift_ref(v, &mut t);
    t
}

fn subst_ref(j: i32, t: &mut Term, s: &BTerm) {
    fn go(j: i32, depth: i32, t: &mut Term, s: &BTerm) {
        match t {
            Term::Idx(i) => {
                if *i - depth == j {
                    // todo remove redundant alloc 
                    *t = *shift(depth, s.clone());
                }
            },
            Term::App(t1, t2) => {
                go(j, depth, t1, s);
                go(j, depth, t2, s);
            },
            Term::Lmb(_, t1) => {
                go(j, depth + 1, t1, s);
            },
        }
    }
    go(j, 0, t, s);
}

fn subst(j: i32, mut t: BTerm, s: &BTerm) -> BTerm {
    subst_ref(j, &mut t, s);
    t
}

fn beta_rule(mut t: BTerm) -> BTerm {
    beta_rule_ref(&mut t);
    t
}

fn beta_rule_ref(t: &mut Term) -> bool {
    match t {
        Term::App(t1, t2) => {
            match t1.as_mut() {
                Term::Lmb(_, t3) => {
                    // todo we can save one pass here
                    shift_ref(1, t2);
                    subst_ref(0, t3, t2);
                    shift_ref(-1, t3);

                    *t = (**t3).clone(); // todo remove clone
                    true
                },
                _ => false
            }
        }
        _ => false
    }
}

pub fn step_norm(t: &mut Term) -> bool {
    match t {
        Term::Idx(_) => false,
        Term::Lmb(_, t1) => {
            step_norm(t1)
        },
        Term::App(t1, t2) => {
            match t1.as_mut() {
                Term::Lmb(_, _) => {
                    // todo inline?
                    beta_rule_ref(t)
                },
                _ => {
                    step_norm(t1) || step_norm(t2)
                }
            }
        }
    }
}

pub fn step_appl(t: &mut Term) -> bool {
    match t {
        Term::Idx(_) => false,
        Term::Lmb(_, t1) => {
            step_appl(t1)
        },
        Term::App(t1, t2) => {
            match t1.as_mut() {
                Term::Lmb(_, _) => {
                    step_appl(t2) || beta_rule_ref(t)
                },
                _ => {
                    step_appl(t1) || step_appl(t2)
                }
            }
        }
    }
}

fn normal_form_ref(step: fn(&mut Term) -> bool, t: &mut Term) {
    let mut i = 0;
    loop {
        println!("{:?}", t);
        if !step(t) || i > 10 {
            break;
        }
        i += 1;
    }
}

pub fn normal_form(step: fn(&mut Term) -> bool, mut t: BTerm) -> BTerm {
    normal_form_ref(step, &mut t);
    return t;
}

// ideas
// * store terms in continous array
// * use rc instead of box to leverage lazy copying
// * interpreter with context map

#[test]
fn test_shift() {
    // Lmb (Lmb (Lmb ((Idx 2 :@: Idx 3) :@: Idx 1)))
    let test = Lmb("x", Lmb("y", Lmb("z", App(App(Idx(2), Idx(3)), Idx(1)))));
    assert_eq!(shift(4, test), Lmb("x", Lmb("y", Lmb("z", App(App(Idx(2), Idx(7)), Idx(1))))))
}

#[test]
fn test_subst() {
    // Lmb ((Idx 2 :@: Idx 0) :@: Idx 1)
    let test = Lmb("x", App(App(Idx(2), Idx(0)), Idx(1)));
    assert_eq!(subst(0, test.clone(), &Idx(0)), Lmb("x", App(App(Idx(2), Idx(0)), Idx(1))));
    assert_eq!(subst(0, test.clone(), &Idx(1)), Lmb("x", App(App(Idx(2), Idx(0)), Idx(2))));
    assert_eq!(subst(0, test.clone(), &Idx(2)), Lmb("x", App(App(Idx(2), Idx(0)), Idx(3))));
}

#[test]
fn test_beta_rule() {
    // ((Lmb $ Lmb $ Idx 0) :@: Idx 41)
    assert_eq!(beta_rule(App(Lmb("x", Lmb("y", Idx(0))), Idx(41))), Lmb("y", Idx(0)));
    assert_eq!(beta_rule(App(Lmb("x", Lmb("y", Idx(1))), Idx(41))), Lmb("y", Idx(42)));
    assert_eq!(beta_rule(App(Lmb("x", Idx(0)), Idx(42))), Idx(42));
}

#[test]
fn test_step_norm() {
    // GHCi> cIDB = Lmb (Idx 0)
    // GHCi> comegaDB = Lmb (Idx 0 :@: Idx 0)
    // GHCi> cOmegaDB = comegaDB :@: comegaDB
    // GHCi> test = cIDB :@: cOmegaDB
    // GHCi> oneStepDBN test == Just cOmegaDB
    let i = Lmb("x", Idx(0));
    let omega = Lmb("x", App(Idx(0), Idx(0)));
    let big_omega = App(omega.clone(), omega);
    let mut test = App(i, big_omega.clone());
    assert_eq!(true, step_norm(&mut test));
    assert_eq!(big_omega, test);
}

#[test]
fn test_step_appl() {
    let i = Lmb("x", Idx(0));
    let omega = Lmb("x", App(Idx(0), Idx(0)));
    let big_omega = App(omega.clone(), omega);
    let mut test = App(i, big_omega.clone());
    let test_before = test.clone();
    assert_eq!(true, step_appl(&mut test));
    assert_eq!(test_before, test);
}

#[allow(non_snake_case)]
#[test]
fn test_normal_form() {
    // GHCi> cIDB = Lmb (Idx 0)
    // GHCi> cKDB = Lmb (Lmb (Idx 1))
    // GHCi> comegaDB = Lmb (Idx 0 :@: Idx 0)
    // GHCi> cOmegaDB = comegaDB :@: comegaDB
    // GHCi> nfDBN (cKDB :@: cIDB :@: cOmegaDB)
    // Lmb (Idx 0)
    // GHCi> nfDBA (cKDB :@: cIDB :@: cOmegaDB)
    let I = Lmb("x", Idx(0));
    let K = Lmb("x", Lmb("y", Idx(1)));
    let ω = Lmb("x", App(Idx(0), Idx(0)));
    let Ω = App(ω.clone(), ω.clone());
    let t = App(App(K.clone(), I.clone()), Ω.clone());
    assert_eq!(Lmb("x", Idx(0)), normal_form(step_norm, t));
}
