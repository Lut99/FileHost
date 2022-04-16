/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:15
 * Last edited:
 *   16 Apr 2022, 16:42:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the CTL executable.
**/

use clap::Parser;

use log::{debug, error, info, LevelFilter};
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use filehost_ctl::cli::{Action, Arguments};


// /***** HELPER MACROS *****/
// /// Logs and prints the same message at the same time.
// macro_rules! log {
//     () => {
//         debug!();
//         println!();
//     };

//     ($fmt_str:literal) => {
//         debug!($fmt_str);
//         println!($fmt_str);
//     };
//     ($fmt_str:literal,$($arg:expr),+) => {
//         debug!($fmt_str, $($arg),+);
//         println!($fmt_str, $($arg),+);
//     };

//     ($severity:ident,$fmt_str:literal) => {
//         $severity!($fmt_str);
//         println!($fmt_str);
//     };
//     ($severity:ident,$fmt_str:literal,$($arg:expr),+) => {
//         $severity!($fmt_str, $($arg),+);
//         println!($fmt_str, $($arg),+);
//     };
// }





/***** ENTRYPOINT *****/
fn main() {
    // Read the CLI
    let args = Arguments::parse();

    // Setup the logger
    TermLogger::init(if args.debug { LevelFilter::Debug } else { LevelFilter::Warn }, Default::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .unwrap_or_else(|err| panic!("Could not create logger: {}", err));
    info!("Initializing FileHost CTL v{}", env!("CARGO_PKG_VERSION"));



    // Connect to the Unix socket



    // Switch on the action
    match args.action {
    }
}
