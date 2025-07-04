mod grammar;

use crate::grammar::program::program;

fn main() {
    println!("{}", program().to_string());
}
