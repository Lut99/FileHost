/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:56:29
 * Last edited:
 *   07 Jun 2022, 12:25:38
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the FileHost server/
**/

use std::fs::File;
use std::io::{Cursor, BufReader, Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

use byteorder::ReadBytesExt;
use clap::Parser;
use log::{debug, error, info, warn};
use nix::sys::select::{FdSet, select};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use systemd::daemon;
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

pub use filehost_srv::errors::ServerError as Error;
use filehost_spc::config::Config;
use filehost_spc::ctl_messages::{ByteOrder, HEALTH_REPLY, Opcode};


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
    let mut origin = args.config_path.display().to_string();
    loop {
        // Call run
        match run(origin, handle) {
            Some((new_origin, new_handle)) => { origin = new_origin; handle = new_handle; },
            None                           => { break; },
        };
    }

    // Done
}



/// Actually runs the daemon. The only reason this is here is to allow reloading easily.
fn run(config_origin: String, mut handle: Box<dyn Read>) -> Option<(String, Box<dyn Read>)> {
    // Read the config file
    let mut config = match Config::from_reader(handle) {
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
    let ctl_socket: UnixListener = unsafe { UnixListener::from_raw_fd(ctl_fd) };

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
                // Get the underlying error
                let err = match ctl_socket.take_error() {
                    Ok(err)  => err,
                    Err(err) => { error!("Could not get CTL socket error: {}", err); continue; }
                };

                // Print it
                error!("{}", Error::CtlSocketError{ fd, err: err.unwrap_or_else(|| panic!("No error found, but the file descriptor did awake on an error")) });
                std::process::exit(1);

            } else {
                warn!("Unknown file descriptor '{}' reports an error; ignoring", fd);
            }
        }

        // Iterate through the triggeted fds which got new data available
        for fd in readfds.fds(None) {
            // Switch on the FD used
            if fd == ctl_fd {
                info!("New data available on CTL stream");

                // Accept the connection
                debug!("Connecting...");
                let (mut stream, address) = match ctl_socket.accept() {
                    Ok(res)  => res,
                    Err(err) => { error!("{}", Error::StreamAcceptError{ what: "CTL", err }); continue; }
                };

                // Wrap in an SSL tunnel
                /* TBD */
                debug!("Established connection with '{:?}'", address);

                // Read the first opcode
                let mut opcode: [u8; 1] = [ 0 ];
                let msg_len: usize = match stream.read(&mut opcode) {
                    Ok(msg_len) => msg_len,
                    Err(err)    => { error!("{}", Error::StreamReadError{ what: "CTL", err }); continue; },
                };
                if msg_len == 0 { error!("{}", Error::EmptyStream{ what: "CTL" }); continue; }

                // Cast the opcode
                let opcode = match Opcode::try_from(opcode[0]) {
                    Ok(opcode) => opcode,
                    Err(err)   => { error!("{}", err); continue; }  
                };

                // Switch on the opcode
                match opcode {
                    Opcode::Health => {
                        // Send the agreed upon constant back
                        if let Err(err) = stream.write(&HEALTH_REPLY) { error!("{}", Error::StreamWriteError{ what: "CTL", err }); continue; }

                        // That's it for health
                        debug!("Handled Health status update");
                    },



                    Opcode::Reload => {
                        // Check if a config is given by reading the next byte
                        let mut config_len: [u8; 2] = [ 0; 2 ];
                        let msg_len: usize = match stream.read(&mut config_len) {
                            Ok(msg_len) => msg_len,
                            Err(err)    => { error!("{}", Error::StreamReadError{ what: "CTL", err }); continue; },
                        };
                        if msg_len == 0 { error!("{}", Error::MissingConfigSize); continue; }

                        // Read as the agreed upon endian
                        let config_len: u16 = Cursor::new(config_len).read_u16::<ByteOrder>().unwrap();

                        // Read the config as the new config if told to do so
                        if config_len > 0 {
                            // Read the subsequent bytes
                            let mut bytes: Vec<u8> = vec![ 0; config_len as usize ];
                            let msg_len: usize = match stream.read(&mut bytes) {
                                Ok(msg_len) => msg_len,
                                Err(err)    => { error!("{}", Error::StreamReadError{ what: "CTL", err }); continue; },
                            };
                            if msg_len != config_len as usize { error!("{}", Error::IncorrectConfigSize{ got: msg_len, expected: config_len as usize }); continue; }

                            // Use that to read a new config
                            config = match Config::from_bytes(bytes) {
                                Ok(config) => config,
                                Err(err)   => { eprintln!("ERROR: {}", Error::ConfigParseError{ origin: "CTL".into(), err }); std::process::exit(1); }
                            };

                        } else {
                            // Use the already given one
                            config = match Config::from_reader(&mut handle) {
                                Ok(config) => config,
                                Err(err)   => { eprintln!("ERROR: {}", Error::ConfigParseError{ origin: config_origin, err }); std::process::exit(1); }
                            };
                        }

                        // Done
                        debug!("Handled Health status update");
                    },
                }

                // Done

            } else {
                warn!("Received message from unknown file descriptor '{}'; ignoring", fd);
            }
        }
    }
}
