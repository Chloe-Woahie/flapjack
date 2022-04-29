#[macro_use]
extern crate prettytable;

mod file_io;
mod flapjack_stack;
mod option_repl;

use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use option_repl::OptionRepl;
use self_update::cargo_crate_version;

// TODO: show last comment on the table (maybe)
// or add a way to check comments without using the log
fn main() {
    match try_update() {
        Ok(_) => {}
        Err(e) => {
            panic!("Binary updated. Please restart the program! ({})", e)
        }
    }
    //.expect_err("Program updated. Please start it again!");
    let path = file_io::init_log_db();

    let stack = FlapJackStackBuilder::from_file(&path).build();
    let repl = OptionRepl::new(stack);
    repl.start();
}

/// If this function returns an Ok, the program updated itself and the binary should likely be restarted
fn try_update() -> Result<(), Box<dyn std::error::Error>> {
    let update_system_build_result = self_update::backends::github::Update::configure()
        .repo_owner("chloe-woahie")
        .repo_name("flapjack")
        .bin_name("flapjack")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build();

    let update_result = match update_system_build_result {
        Ok(update_system) => update_system.update(),
        Err(e) => {
            panic!("Failed to build update system! ({})", e);
        }
    };

    match update_result {
        // If update_result is Ok, then the updater updated the program
        Ok(_) => Ok(()),
        Err(e) => {
            // Update Errors are allowed because they mean the user decided not to update
            match e {
                self_update::errors::Error::Update(_) => {}
                _ => panic!("Updater failed! ({})", e),
            }

            Err("Program did not update.".into())
        }
    }
}
