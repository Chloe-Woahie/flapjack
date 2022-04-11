#[macro_use]
extern crate prettytable;
mod file_io;
mod flapjack_stack;
mod option_repl;

use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use option_repl::OptionRepl;

fn main() {
    let path = file_io::init_log_db();

    // TODO: tell the user where they're loading from
    let stack = FlapJackStackBuilder::from_file(&path).build();
    let repl = OptionRepl::new(stack);
    repl.start();
}
