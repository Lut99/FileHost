/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   07 Jun 2022, 12:18:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::os::unix::io::RawFd;

use filehost_spc::ctl_messages::Opcode;


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
    /// The CTL socket stream has errored.
    CtlSocketError{ fd: RawFd, err: std::io::Error },
    /// Some file descriptor has become invalid
    FdError{ what: &'static str, fd: RawFd },

    /// Could not accept a new connection.
    StreamAcceptError{ what: &'static str, err: std::io::Error },
    /// The given stream was notified but empty.
    EmptyStream{ what: &'static str },
    /// Could not read from the given stream.
    StreamReadError{ what: &'static str, err: std::io::Error },
    /// Could not write to the given stream.
    StreamWriteError{ what: &'static str, err: std::io::Error },

    /// The given 'Reload' command did not have a config size specified.
    MissingConfigSize,
    /// The given config was not the promised amount of bytes.
    IncorrectConfigSize{ got: usize, expected: usize },
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ServerError::*;
        match self {
            ConfigParseError{ origin, err } => write!(f, "Could not parse configuration file '{}': {}", origin, err),

            ListenFdsFailure{ err } => write!(f, "Could not get list of file descriptors: {}", err),

            SelectError{ err }            => write!(f, "Could not select on sockets: {}", err),
            CtlSocketError{ fd, err }     => write!(f, "An error has occurred on the CTL socket ({}): {}", fd, err),
            FdError{ what, fd }           => write!(f, "{} file descriptor ({}) has become invalid", what, fd),

            StreamAcceptError{ what, err } => write!(f, "Could not accept new connection on {} stream: {}", what, err),
            EmptyStream{ what }            => write!(f, "{} stream is woken up but empty", what),
            StreamReadError{ what, err }   => write!(f, "Could not read from {} stream: {}", what, err),
            StreamWriteError{ what, err }  => write!(f, "Could not write to {} stream: {}", what, err),

            MissingConfigSize                    => write!(f, "Received {} without config size", Opcode::Reload),
            IncorrectConfigSize{ got, expected } => write!(f, "Given config file was said to have {} bytes, but got {}", expected, got),
        }
    }
}

impl Error for ServerError {}
