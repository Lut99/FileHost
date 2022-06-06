/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:56:29
 * Last edited:
 *   06 Jun 2022, 13:28:21
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the FileHost server/
**/

use std::fs::File;
use std::io::{BufReader, Read};
use std::net::UdpSocket;
use std::os::unix::io::{FromRawFd, RawFd};
use std::path::PathBuf;

use clap::Parser;
use log::{debug, error, info, warn};
use nix::sys::select::{FdSet, select};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use systemd::daemon;
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

use filehost_spc::config::Config;

pub use filehost_srv::errors::ServerError as Error;


/***** CLI *****/
/// Contains the command-line / environment variable arguments for the daemon.
#[derive(Parser)]
struct Args {
    /// The location of the config file, from which the rest will be read.
    #[clap(short, long, default_value = "/etc/filehost/config.json", help = "The location of the configuration JSON file. Any other settings will be read from there.", env = "CONFIG_PATH")]
    config_path : PathBuf,
}





/***** ENTRYPOINT *****/
fn main() {
    // First, parse the CLI to see if we need another config path
    let args = Args::parse();

    // Open the file in buffered mode
    let mut handle: Box<dyn Read> = match File::open(&args.config_path) {
        Ok(handle) => Box::new(BufReader::new(handle)),
        Err(err)   => { eprintln!("ERROR: Could not open config file '{}': {}", args.config_path.display(), err); std::process::exit(1); }
    };

    // Now pass everything to the 'run' function to handle the rest (and allow reloads with different config files).
    loop {
        // Call run
        match run(args.config_path.display().to_string(), handle) {
            Some(new_handle) => { handle = new_handle; },
            None             => { break; },
        };
    }

    // Done
}



/// Actually runs the daemon. The only reason this is here is to allow reloading easily.
fn run(config_origin: String, handle: Box<dyn Read>) -> Option<Box<dyn Read>> {
    // Read the config file
    let config = match Config::from_reader(handle) {
        Ok(config) => config,
        Err(err)   => { eprintln!("ERROR: {}", Error::ConfigParseError{ origin: config_origin, err }); std::process::exit(1); }
    };

    // Prepare the logger(s)
    if connected_to_journal() {
        // We are connected to systemd, so create that logger
        init_with_extra_fields(vec![
            ("VERSION", env!("CARGO_PKG_VERSION"))
        ]).unwrap_or_else(|err| panic!("Could not create systemd logger: {}", err));
        log::set_max_level(config.log_level);
    } else {
        // We are not connected, so just use the terminal logger
        TermLogger::init(config.log_level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto)
            .unwrap_or_else(|err| panic!("Could not create stderr logger: {}", err));
    }
    info!("Initializing FileHost Server v{}", env!("CARGO_PKG_VERSION"));
    debug!("Config path: '{}'", config_origin);



    // Open a connection to the CTL unix socket
    let fds = match daemon::listen_fds(false) {
        Ok(fds)  => fds,
        Err(err) => { error!("{}", Error::ListenFdsFailure{ err }); std::process::exit(1); }
    };

    // Open a stream around it
    if fds.len() != 1 { panic!("Got more file descriptors than bargained for??? (got {}, expected 1)", fds.len()) }
    let ctl_fd: RawFd = fds.iter().next().unwrap();
    let ctl_stream: UdpSocket = unsafe { UdpSocket::from_raw_fd(ctl_fd) };

    // Next, open a stream around the network socket
    /* TBD */



    // Main wait loop!
    info!("Listening on sockets...");
    loop {
        // Collect the file descriptors in a set
        let mut readfds = FdSet::new();
        readfds.insert(ctl_fd);

        // Create an error set for that set
        let mut errorfds = readfds.clone();

        // Switch on the first one to become available
        if let Err(err) = select(None, &mut readfds, None, &mut errorfds, None) {
            error!("{}", Error::SelectError{ err });
            continue;
        }

        // Iterate through streams who may have crashed
        for fd in errorfds.fds(None) {
            // Switch on the FD used
            if fd == ctl_fd {
                // Check for any errors
                error!("{}", Error::FdError{ what: "CTL", fd });
                std::process::exit(1);

            } else {
                warn!("Unknown file descriptor '{}' reports an error; ignoring", fd);
            }
        }

        // Iterate through the triggeted fds which got new data available
        for fd in readfds.fds(None) {
            // Switch on the FD used
            if fd == ctl_fd {
                debug!("New data available on CTL stream");

                // Read it
                let mut buffer: [u8; 1024] = [0; 1024];
                let msg_len: usize = match ctl_stream.recv(&mut buffer) {
                    Ok(msg_len) => msg_len,
                    Err(err)    => { error!("{}", Error::StreamReadError{ what: "CTL", err }); continue; },
                };

                // Simply print whatever they send us
                info!("Message: '{}'", String::from_utf8_lossy(&buffer[..msg_len]).to_string());

                // Done

            } else {
                warn!("Received message from unknown file descriptor '{}'; ignoring", fd);
            }
        }
    }
}
