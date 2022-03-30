/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:15
 * Last edited:
 *   30 Mar 2022, 20:54:15
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the server executable.
**/

use std::fs::File;

use clap::Parser;

use filehost_ctl::cli::Arguments;
use log::info;
use simplelog::{ColorChoice, TermLogger, TerminalMode, WriteLogger};
#[cfg(unix)]
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};


/***** ENTRYPOINT *****/
fn main() {
    // Read the CLI
    let args = Arguments::parse();

    // Setup the correct logger
    if let Some(path) = args.log_file {
        // Write to the file
        WriteLogger::init(args.log_level, Default::default(), File::create(&path).unwrap_or_else(|err| panic!("Could not create logging file '{}': {}", path.display(), err)))
            .unwrap_or_else(|err| panic!("Could not create file logger: {}", err));

    } else {
        // Check if we can use the systemd logger
        #[cfg(unix)]
        let use_systemd_logger = connected_to_journal();
        #[cfg(not(unix))]
        let use_systemd_logger = false;

        // If it's enabled, use it
        #[cfg(unix)]
        if use_systemd_logger {
            // We are connected to systemd, so create that logger
            init_with_extra_fields(vec![
                ("VERSION", env!("CARGO_PKG_VERSION"))
            ]).unwrap_or_else(|err| panic!("Could not create systemd logger: {}", err));
            log::set_max_level(args.log_level);
        }

        // If it's not enabled, use the normal logger
        if !use_systemd_logger {
            TermLogger::init(args.log_level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto)
                .unwrap_or_else(|err| panic!("Could not create stderr logger: {}", err));
        }
    }
    info!("Initializing FileHost Server v{}", env!("CARGO_PKG_VERSION"));



    // Switch on the action
    match args.action {
        
    }
}
