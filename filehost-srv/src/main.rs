/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:56:29
 * Last edited:
 *   11 Jun 2022, 15:32:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the FileHost server/
**/

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

use clap::Parser;
use log::{debug, error, info, warn};
use nix::sys::select::{FdSet, select};
use rustls::{OwnedTrustAnchor, RootCertStore};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use systemd::daemon;
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

use filehost_spc::config::Config;
use filehost_spc::ctl_messages::{ByteOrder, HEALTH_REPLY, Opcode};
use filehost_spc::login::ROOT_ID;

pub use filehost_srv::errors::ServerError as Error;
use filehost_srv::users::{User, Users};
use filehost_srv::ssl::SSLConfig;


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
    let handle: BufReader<File> = match File::open(&args.config_path) {
        Ok(handle) => BufReader::new(handle),
        Err(err)   => { eprintln!("ERROR: Could not open config file '{}': {}", args.config_path.display(), err); std::process::exit(1); }
    };
    // Read the config file
    let config = match Config::from_reader(handle) {
        Ok(config) => config,
        Err(err)   => { eprintln!("ERROR: {}", Error::ConfigParseError{ path: args.config_path, err }); std::process::exit(1); }
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
    debug!("Config path: '{}'", args.config_path.display());



    // Read the database file
    info!("Loading users...");
    debug!("User database: '{}'", config.user_db.display());
    let users: Users = match Users::from_file(&config.user_db) {
        Ok(users) => users,
        Err(err)  => { error!("{}", Error::UsersParseError{ path: config.user_db, err }); std::process::exit(1); }
    };

    // Prepare the SSL Config
    info!("Initializing SSL...");
    let ssl_conf: SSLConfig = match SSLConfig::new(&config.server_cert, &config.server_key, &users) {
        Ok(users) => users,
        Err(err)  => { error!("{}", Error::SSLConfigError{ err }); std::process::exit(1); }  
    };



    // Prepare the certificate store
    info!("Preparing certificate store...");
    let mut root_store = RootCertStore::empty();
    debug!("Loading Mozilla certificates...");
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));



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
            // Determine the user for this session
            let user: &User = if fd == ctl_fd {
                // The user is the root user
                users.users.get(&ROOT_ID).expect("No Root user in users database; this should never happen!")
            } else {
                warn!("Unknown file descriptor '{}' is ready for reading; ignoring", fd);
                continue;
            };

            // Accept the connection
            debug!("Accepting new connection...");
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
            }

            // Done
        }
    }

    // Done
}

