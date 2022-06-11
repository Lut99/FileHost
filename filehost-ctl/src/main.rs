/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:15
 * Last edited:
 *   11 Jun 2022, 15:22:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the CTL executable.
**/

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use clap::Parser;
use log::{debug, error, info, LevelFilter};
use simplelog::{ColorChoice, TermLogger, TerminalMode};

pub use filehost_ctl::errors::CtlError as Error;
use filehost_ctl::cli::{Action, Arguments};
use filehost_spc::config::Config;
use filehost_spc::ctl_messages::{HEALTH_REPLY, Opcode};


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

    // Read the config file
    let config = match Config::from_file(&args.config_path) {
        Ok(config) => config,
        Err(err)   => { error!("{}", err); std::process::exit(1); }  
    };



    // Connect to the Unix socket
    debug!("Connecting to: {}...", &config.socket_path.display());
    let mut conn = match UnixStream::connect(&config.socket_path) {
        Ok(conn) => conn,
        Err(err) => { error!("{}", Error::SocketConnectError{ addr: config.socket_path, err }); std::process::exit(1); }
    };



    // Switch on the action
    match args.action {
        Action::Health{} => {
            info!("Checking server health status...");

            // Send a message to the server
            debug!("Sending '{:?}' to server...", Opcode::Health);
            if let Err(err) = conn.write(&[ Opcode::Health as u8 ]) { error!("{}", Error::SocketWriteError{ err }); std::process::exit(1); }

            // Wait for a response
            let mut buffer: [u8; HEALTH_REPLY.len()] = [ 0; HEALTH_REPLY.len() ];
            if let Err(err) = conn.read(&mut buffer) { error!("{}", Error::SocketReadError{ err }); std::process::exit(1); }

            // Compare it
            debug!("Checking server reply...");
            if buffer != HEALTH_REPLY {
                error!("Server replied, but with incorrect response:\n\n    Expected:\n     > {:?}\n\n    Got:\n     > {:?}\n", HEALTH_REPLY, buffer);
                std::process::exit(1);
            }

            // Otherwise, success
            println!("Server OK");
        },
    }



    // Done!
    info!("Done.");
}
