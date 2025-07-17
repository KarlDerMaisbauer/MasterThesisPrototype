mod grammar;

use crate::grammar::program::program;

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    println!("{}", program().to_string());
}
