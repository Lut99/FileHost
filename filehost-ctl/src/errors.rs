/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   06 Jun 2022, 13:09:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;


/***** ERRORS *****/
/// General toplevel errors.
#[derive(Debug)]
pub enum CtlError {
    /// Failed to create a new socket.
    SocketCreateError{ err: nix::errno::Errno },
    /// Failed to connect to the given socket.
    SocketConnectError{ path: PathBuf, err: nix::errno::Errno },

    /// Could not write to the server stream.
    SocketWriteError{ err: std::io::Error },
    /// Could not flush the server stream.
    SocketFlushError{ err: std::io::Error },
}

impl Display for CtlError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use CtlError::*;
        match self {
            SocketCreateError{ err }        => write!(f, "Could not create Unix socket: {}", err),
            SocketConnectError{  path, err} => write!(f, "Could not connect to server socket '{}': {}", path.display(), err),

            SocketWriteError{ err }        => write!(f, "Could not write to server socket: {}", err),
            SocketFlushError{ err }        => write!(f, "Could not flush server socket: {}", err),
        }
    }
}

impl Error for CtlError {}





// /// Errors relating to the health action.
// #[derive(Debug)]
// pub enum HealthError {
//     /// Could not 
// }
