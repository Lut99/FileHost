/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   06 Jun 2022, 13:25:14
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::os::unix::io::RawFd;


/***** ERRORS *****/
/// Errors that relate to the root part of the server.
#[derive(Debug)]
pub enum ServerError {
    /// Could not parse the configuration file
    ConfigParseError{ origin: String, err: filehost_spc::config::Error },

    /// Could not get the list of file descriptors from systemd.
    ListenFdsFailure{ err: systemd::Error },

    /// Could not wait for any socket to become available.
    SelectError{ err: nix::Error },
    /// Some file descriptor has become invalid
    FdError{ what: &'static str, fd: RawFd },
    /// Could not accept a new connection.
    AcceptError{ what: &'static str, err: std::io::Error },
    /// Could not read from the given stream.
    StreamReadError{ what: &'static str, err: std::io::Error },
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ServerError::*;
        match self {
            ConfigParseError{ origin, err } => write!(f, "Could not parse configuration file '{}': {}", origin, err),

            ListenFdsFailure{ err } => write!(f, "Could not get list of file descriptors: {}", err),

            SelectError{ err }            => write!(f, "Could not select on sockets: {}", err),
            FdError{ what, fd }           => write!(f, "{} file descriptor ({}) has become invalid", what, fd),
            AcceptError{ what, err }      => write!(f, "Could not accept new connection on {} stream: {}", what, err),
            StreamReadError{ what, err }  => write!(f, "Could not read from {} stream: {}", what, err),
        }
    }
}

impl Error for ServerError {}
