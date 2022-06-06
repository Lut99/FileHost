/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   06 Jun 2022, 15:32:49
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
    SocketConnectError{ addr: PathBuf, err: std::io::Error },

    /// Could not read from the server stream.
    SocketReadError{ err: std::io::Error },
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
            SocketConnectError{ addr, err } => write!(f, "Could not create & connect Unix socket to '{}': {}", addr.display(), err),

            SocketReadError{ err }  => write!(f, "Could not read from server socket: {}", err),
            SocketWriteError{ err } => write!(f, "Could not write to server socket: {}", err),
            SocketFlushError{ err } => write!(f, "Could not flush server socket: {}", err),
        }
    }
}

impl Error for CtlError {}





// /// Errors relating to the health action.
// #[derive(Debug)]
// pub enum HealthError {
//     /// Could not 
// }
