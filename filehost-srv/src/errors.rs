/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   11 Jun 2022, 15:24:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the crate.
**/

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult, Write};
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use filehost_spc::login::UserId;


/***** ERRORS *****/
/// Errors that relate to the root part of the server.
#[derive(Debug)]
pub enum ServerError {
    /// Could not parse the configuration file
    ConfigParseError{ path: PathBuf, err: filehost_spc::config::Error },
    /// Could not load the users database
    UsersParseError{ path: PathBuf, err: UserError },
    /// Could not prepare the SSL config
    SSLConfigError{ err: SSLError },

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
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ServerError::*;
        match self {
            ConfigParseError{ path, err } => write!(f, "Could not parse configuration file '{}': {}", path.display(), err),
            UsersParseError{ path, err }  => write!(f, "Could not parse users database '{}': {}", path.display(), err),
            SSLConfigError{ err }         => write!(f, "Could not initialize SSL config: {}", err),

            ListenFdsFailure{ err } => write!(f, "Could not get list of file descriptors: {}", err),

            SelectError{ err }            => write!(f, "Could not select on sockets: {}", err),
            CtlSocketError{ fd, err }     => write!(f, "An error has occurred on the CTL socket ({}): {}", fd, err),
            FdError{ what, fd }           => write!(f, "{} file descriptor ({}) has become invalid", what, fd),

            StreamAcceptError{ what, err } => write!(f, "Could not accept new connection on {} stream: {}", what, err),
            EmptyStream{ what }            => write!(f, "{} stream is woken up but empty", what),
            StreamReadError{ what, err }   => write!(f, "Could not read from {} stream: {}", what, err),
            StreamWriteError{ what, err }  => write!(f, "Could not write to {} stream: {}", what, err),
        }
    }
}

impl Error for ServerError {}



/// Errors that relate to SSL & encryption and such.
#[derive(Debug)]
pub enum SSLError {
    /// Could not read the server certificate file.
    CertOpenError{ path: PathBuf, err: std::io::Error },
    /// Failed to parse a user certificate(s) string.
    CertParseError{ path: String, err: std::io::Error },
    /// Could not open the server key file.
    KeyOpenError{ path: PathBuf, err: std::io::Error },
    /// The given keys file is empty.
    NoKeysFound{ path: PathBuf },
    /// Failed to parse the given key file.
    KeyParseError{ path: PathBuf, err: std::io::Error },
    /// Could not add a new client key to the global list
    CertAddError{ err: webpki::Error },
    /// Failed to build the server config.
    ConfigError{ err: rustls::Error },
}

impl Display for SSLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SSLError::*;
        match self {
            CertOpenError{ path, err }  => write!(f, "Could not open certificate(s) file '{}': {}", path.display(), err),
            CertParseError{ path, err } => write!(f, "Could not parse certificate(s) file '{}': {}", path, err),
            KeyOpenError{ path, err }   => write!(f, "Could not open server key file '{}': {}", path.display(), err),
            NoKeysFound{ path }         => write!(f, "Key file '{}' does not contain any keys", path.display()),
            KeyParseError{ path, err }  => write!(f, "Could not parse key file '{}': {}", path.display(), err),
            CertAddError{ err }         => write!(f, "Failed to add user key to root store: {}", err),
            ConfigError{ err }          => write!(f, "Failed to create server SSL/TLS config: {}", err),
        }
    }
}

impl Error for SSLError {}



/// Errors that relate to interaction with the User database / logging in.
#[derive(Debug)]
pub enum UserError {
    /// The given file could not be opened.
    FileOpenError{ path: PathBuf, err: std::io::Error },
    /// Could not get the file's metadata.
    MetadataError{ path: PathBuf, err: std::io::Error },
    /// The given file has the incorrect permissions
    IncorrectPermissions{ path: PathBuf, got: [u8; 3], expected: [u8; 3] },
    /// Failed to parse the users database file.
    FileParseError{ path: PathBuf, err: serde_json::Error },
    /// Missing one of the reserved users.
    ReservedUserError{ path: PathBuf, id: UserId },
}

impl UserError {
    /// Prints the given octet of permissions in a Debug-way.
    #[inline]
    fn octet_debug(octets: &[u8; 3]) -> Result<String, std::fmt::Error> { Ok(format!("{}{}{}", octets[0], octets[1], octets[2])) }

    /// Prints the given octet of permissions in a Display-way.
    fn octet_display(octets: &[u8; 3]) -> Result<String, std::fmt::Error> {
        // Extract a set of permissions from each octet
        let mut result: String = String::with_capacity(17);
        let user_id: [char; 3] = [ 'u', 'g', 'o' ];
        for (i, oct) in octets.iter().enumerate() {
            // Check the permissions
            let read : bool = (oct & 0b100) != 0;
            let write: bool = (oct & 0b010) != 0;
            let exec : bool = (oct & 0b001) != 0;

            // Use that to write the string
            if read || write || exec {
                // Write a comma if necessary
                if !result.is_empty() { write!(&mut result, ",")?; }

                // Write the permission string
                write!(&mut result, "{}+{}{}{}",
                    user_id[i],
                    if read { "r" } else { "" },
                    if write { "w" } else { "" },
                    if exec { "x" } else { "" },
                )?;
            }
        }

        // Done
        Ok(result)
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use UserError::*;
        match self {
            FileOpenError{ path, err }                  => write!(f, "Could not open file '{}': {}", path.display(), err),
            MetadataError{ path, err }                  => write!(f, "Could not get metadata of file '{}': {}", path.display(), err),
            IncorrectPermissions{ path, got, expected } => write!(f, "File '{}' has insecure permissions set (got {} ({:?}), expected {} ({:?}))", path.display(), Self::octet_display(got)?, Self::octet_debug(got)?, Self::octet_display(expected)?, Self::octet_debug(expected)?),
            FileParseError{ path, err }                 => write!(f, "Could not parse file '{}': {}", path.display(), err),
            ReservedUserError{ path, id }               => write!(f, "File '{}' is missing reserved user with ID {}", path.display(), id),
        }
    }
}

impl Error for UserError {}
