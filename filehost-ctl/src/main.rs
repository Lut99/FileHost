/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:15
 * Last edited:
 *   01 Apr 2022, 14:01:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the CTL executable.
**/

use clap::Parser;

use filehost_ctl::cli::{Action, Arguments};
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, TermLogger, TerminalMode};


/***** HELPER MACROS *****/
/// Logs and prints the same message at the same time.
macro_rules! log {
    () => {
        debug!();
        println!();
    };

    ($fmt_str:literal) => {
        debug!($fmt_str);
        println!($fmt_str);
    };
    ($fmt_str:literal,$($arg:expr),+) => {
        debug!($fmt_str, $($arg),+);
        println!($fmt_str, $($arg),+);
    };

    ($severity:ident,$fmt_str:literal) => {
        $severity!($fmt_str);
        println!($fmt_str);
    };
    ($severity:ident,$fmt_str:literal,$($arg:expr),+) => {
        $severity!($fmt_str, $($arg),+);
        println!($fmt_str, $($arg),+);
    };
}





/***** ENTRYPOINT *****/
fn main() {
    // Read the CLI
    let args = Arguments::parse();

    // Setup the logger
    TermLogger::init(if args.debug { LevelFilter::Debug } else { LevelFilter::Error }, Default::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .unwrap_or_else(|err| panic!("Could not create logger: {}", err));
    info!("Initializing FileHost Server v{}", env!("CARGO_PKG_VERSION"));



    // Switch on the action
    match args.action {
        Action::Install{ server_exec, config_path, socket_path } => {
            debug!("Running 'install' command...");

            // First, download the server executable (if needed)
            let server_exec = match server_exec {
                Some(server_exec) => server_exec,
                None => {
                    // Create a temporary directory
                    
                }
            };

            // First, create the systemd folder
            log!("Creating server systemd entry...");
        },

        Action::Uninstall{ config_path } => {
            debug!("Running 'uninstall' command...");

            // Finally, remove the systemd folder
            
        },
    }
}
