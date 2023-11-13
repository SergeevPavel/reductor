
#[derive(Debug, Eq, PartialEq, Clone)]
enum Term {
    Idx(i32),
    App(Box<Term>, Box<Term>),
    Lmb(Box<Term>),
}

type BTerm = Box<Term>;

#[allow(non_snake_case)]
fn Idx(i: i32) -> BTerm {
    Box::new(Term::Idx(i))
}

#[allow(non_snake_case)]
fn App(t1: BTerm, t2: BTerm) -> BTerm {
    Box::new(Term::App(t1, t2))
}

#[allow(non_snake_case)]
fn Lmb(t: BTerm) -> BTerm {
    Box::new(Term::Lmb(t))
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
            Term::Lmb(t1) => {
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
            Term::Lmb(t1) => {
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

fn beta_rule(t: BTerm) -> BTerm {
    match *t {
        Term::App(t1, t2) => {
            match *t1 {
                Term::Lmb(t3) => {
                    // todo we can save one pass here
                    shift(-1, subst(0, t3, &shift(1, t2)))
                },
                _ => unreachable!("Can't apply {:?} to  {:?}", t1, t2)
            }
        }
        _ => t
    }
}
//
//fn beta_rule_ref(t: &mut Term) -> bool {
//    
//}

//fn step_norm(t: &mut Term) -> bool {
//    match t {
//        Term::Idx(_) => false,
//        Term::Lmb(t1) => {
//            step_norm(t1)
//        },
//        Term::App(t1, t2) => {
//            match t1 {
//                Term::Lmb(t3) => {
//                    
//                    *t1 = shift(-1, subst(0, *t3, &shift(1, *t2)));
//                    true
//                },
//                _ => {
//                    todo!()
//                }
//            }
//        }
//    }
//}

fn step_appl() {
    
}

// ideas
// * store terms in continous array
// * use rc instead of box to leverage lazy copying
// * interpreter with context map

#[test]
fn test_shift() {
    // Lmb (Lmb (Lmb ((Idx 2 :@: Idx 3) :@: Idx 1)))
    let test = Lmb(Lmb(Lmb(App(App(Idx(2), Idx(3)), Idx(1)))));
    assert_eq!(shift(4, test), Lmb(Lmb(Lmb(App(App(Idx(2), Idx(7)), Idx(1))))))
}

#[test]
fn test_subst() {
    // Lmb ((Idx 2 :@: Idx 0) :@: Idx 1)
    let test = Lmb(App(App(Idx(2), Idx(0)), Idx(1)));
    assert_eq!(subst(0, test.clone(), &Idx(0)), Lmb(App(App(Idx(2), Idx(0)), Idx(1))));
    assert_eq!(subst(0, test.clone(), &Idx(1)), Lmb(App(App(Idx(2), Idx(0)), Idx(2))));
    assert_eq!(subst(0, test.clone(), &Idx(2)), Lmb(App(App(Idx(2), Idx(0)), Idx(3))));
}

#[test]
fn test_beta_rule() {
    // ((Lmb $ Lmb $ Idx 0) :@: Idx 41)
    assert_eq!(beta_rule(App(Lmb(Lmb(Idx(0))), Idx(41))), Lmb(Idx(0)));
    assert_eq!(beta_rule(App(Lmb(Lmb(Idx(1))), Idx(41))), Lmb(Idx(42)));
    assert_eq!(beta_rule(App(Lmb(Idx(0)), Idx(42))), Idx(42));
}

fn main() {
    let f = Lmb(App(Idx(0), Idx(0))); // ƛx.xx
    println!("{:?}", f);
}
