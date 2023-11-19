mod term;
mod expr;
mod pars;
mod prn;

use term::*;

fn main() {
    let f = Lmb("x", App(Idx(0), Idx(0))); // ƛx.xx
    // (fn [x] (x x))
    println!("{:?}", f);
}
