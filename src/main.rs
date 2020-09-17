mod basics;
mod var;
mod term;

use term::Term;

fn main() {
    let x = Term::from("4+3*x^(3*y/2)");
    println!("{}", x.to_string())
}
