mod term;
mod expr;

use term::*;

fn main() {
    let f = Lmb("x", App(Idx(0), Idx(0))); // ƛx.xx
    println!("{:?}", f);
}
