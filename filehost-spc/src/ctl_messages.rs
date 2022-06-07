/* CTL MESSAGES.rs
 *   by Lut99
 *
 * Created:
 *   06 Jun 2022, 14:22:44
 * Last edited:
 *   07 Jun 2022, 12:11:12
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the layout & opcodes for messages between the CTL and the
 *   daemon.
**/

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};


/***** CONSTANTS *****/
/// Defines the byte order used when transmitting numbers over sockets.
pub type ByteOrder = byteorder::BigEndian;

/// Defines the message to be send in response of the health message.
pub const HEALTH_REPLY: [u8; 1] = [ 42 ];





/***** ERRORS *****/
#[derive(Debug)]
pub enum OpcodeError {
    /// Encountered an illegal value
    UnknownValue{ raw: u8 },
}

impl Display for OpcodeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use OpcodeError::*;
        match self {
            UnknownValue{ raw } => write!(f, "Encountered unknown Opcode '{}'", raw),
        }
    }
}

impl Error for OpcodeError {}





/***** ENUMS *****/
/// Defines operational opcodes.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    /// Asks the server if it's alive
    Health = 0,

    /// Reloads the server (restart but not really), possibly with a new config
    Reload = 1,
}

impl Debug for Opcode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", *self as u8)
    }
}

impl Display for Opcode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Opcode::*;
        match self {
            Health => write!(f, "Opcode::Health"),

            Reload => write!(f, "Opcode::Reload"),
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == u8::from(Opcode::Health) { Ok(Opcode::Health) }
        else if value == u8::from(Opcode::Reload) { Ok(Opcode::Reload) }
        else { Err(OpcodeError::UnknownValue{ raw: value }) }
    }
}

impl From<Opcode> for u8 {
    #[inline]
    fn from(value: Opcode) -> Self {
        value as u8
    }
}
