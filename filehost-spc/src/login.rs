/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   11 Jun 2022, 11:28:06
 * Last edited:
 *   11 Jun 2022, 12:34:55
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains specification enums & structs for logging in.
**/

use std::fmt::{Display, Formatter, Result as FResult};
use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};


/***** CONSTANTS *****/
/// The constant ID of the root user.
pub const ROOT_ID : UserId = 0;

/// The constant ID of the guest user.
pub const GUEST_ID : UserId = 1;





/***** TYPES *****/
/// The type wrapper we use for user IDs.
pub type UserId = u64;





/***** FLAGS *****/
/// Defines the account permissions.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Permissions(u8);

impl Permissions {
    /// Shortcut for no permissions.
    pub const NONE : Self = Self(0x00);
    /// Shortcut for all permissions.
    pub const ALL  : Self = Self(0xFF);


    /// Returns whether this user has (at least) the given set of permissions.
    #[inline]
    pub fn has<P: Into<u8>>(&self, req: P) -> bool { let req: u8 = req.into(); (self.0 & req) == req }
}

impl BitOr for Permissions {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Permissions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOr<u8> for Permissions {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<u8> for Permissions {
    #[inline]
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}

impl From<u8> for Permissions {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Permissions> for u8 {
    #[inline]
    fn from(value: Permissions) -> Self {
        value.0
    }
}

impl Display for Permissions {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        Ok(())
    }
}
