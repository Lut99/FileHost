/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:15
 * Last edited:
 *   06 Jun 2022, 13:21:03
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the CTL executable.
**/

use std::io::{BufWriter, Write};
use std::net::UdpSocket;
use std::os::unix::io::{FromRawFd, RawFd};

use clap::Parser;
use log::{debug, error, info, LevelFilter};
use nix::sys::socket::{connect, socket, AddressFamily, SockFlag, SockType, SockProtocol, UnixAddr};
use simplelog::{ColorChoice, TermLogger, TerminalMode};

pub use filehost_ctl::errors::CtlError as Error;
use filehost_ctl::cli::{Action, Arguments};
use filehost_spc::config::Config;


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
    debug!("Creating Unix socket...");
    let sock: RawFd = match socket(AddressFamily::Unix, SockType::Datagram, SockFlag::empty(), None) {
        Ok(sock) => sock,
        Err(err) => { error!("{}", Error::SocketCreateError{ err }); std::process::exit(1); }
    };
    debug!("Connecting to: {}...", &config.socket_path.display());
    if let Err(err) = connect(sock, &UnixAddr::new(&config.socket_path).unwrap()) {
        error!("{}", Error::SocketConnectError{ path: config.socket_path, err });
        std::process::exit(1);
    }

    // Wrap it in a stream
    let conn: UdpSocket = unsafe { UdpSocket::from_raw_fd(sock) };
    debug!("Connection success");



    // Switch on the action
    match args.action {
        Action::Health{} => {
            info!("Checking server health status...");

            // Send a message to the server
            debug!("Sending 'Hello, world!' to server...");
            if let Err(err) = conn.send("Hello, world!".as_ref()) { error!("{}", Error::SocketWriteError{ err }); }
        },
    }



    // Done!
    info!("Done.");
}
