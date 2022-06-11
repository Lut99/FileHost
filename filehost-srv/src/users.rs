/* USERS.rs
 *   by Lut99
 *
 * Created:
 *   11 Jun 2022, 11:24:04
 * Last edited:
 *   11 Jun 2022, 15:02:01
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines an interface to some user database.
**/

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

use serde::{Deserialize, Serialize};

use filehost_spc::login::{GUEST_ID, ROOT_ID, Permissions, UserId};

pub use crate::errors::UserError as Error;


/***** CONSTANTS *****/
/// The permissions that we require on the users file.
const USER_PERMISSIONS : [u8; 3] = [ 6, 0, 0 ];




/***** HELPER TRAITS *****/
/// Determines if a type may be formatted using the PermissionsFormatter.
pub trait PermissionsOctet: PermissionsExt {
    /// Returns the relevant octets of information.
    fn get_octets(&self) -> [u8; 3] {
        // Get the mode
        let mode: u32 = self.mode();

        // Return an array with the correct bits
        [
            ((mode >> 6) & 0x7) as u8,
            ((mode >> 3) & 0x7) as u8,
            ( mode       & 0x7) as u8,
        ]
    }
}

impl<T: PermissionsExt> PermissionsOctet for T {}





/***** LIBRARY *****/
/// A JSON struct that contains the global user database.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Users {
    /// The highest UserId used.
    #[serde(skip)]
    pub max_id : UserId,
    /// The list of Users.
    pub users  : HashMap<UserId, User>,
}

impl Users {
    /// Constructor for the Users database, which loads it from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Turn the Path-like into a Path
        let path: &Path = path.as_ref();

        // Try to open the file
        let handle = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileOpenError{ path: path.into(), err }); }
        };

        // Check the file permissions
        let perms: [u8; 3] = match handle.metadata() {
            Ok(metadata) => metadata.permissions().get_octets(),
            Err(err)     => { return Err(Error::MetadataError{ path: path.into(), err }); }
        };
        if perms != USER_PERMISSIONS { return Err(Error::IncorrectPermissions{ path: path.into(), got: perms, expected: USER_PERMISSIONS }) }

        // If it _does_ check out, read with Serde
        let mut res: Users = match serde_json::from_reader(BufReader::new(handle)) {
            Ok(res)  => res,
            Err(err) => { return Err(Error::FileParseError{ path: path.into(), err }); }
        };

        // Count the highest user
        let (mut root_found, mut guest_found) = (false, false);
        res.max_id = 0;
        for (id, user) in &mut res.users {
            // Check if we find any of the reserved users
            if *id == ROOT_ID  { root_found = true; }
            if *id == GUEST_ID { guest_found = true; }

            // Update the ID in the user thingy
            user.id = *id;

            // Check the max
            if user.id > res.max_id { res.max_id = user.id }
        }
        if !root_found  { return Err(Error::ReservedUserError{ path: path.into(), id: ROOT_ID }); }
        if !guest_found { return Err(Error::ReservedUserError{ path: path.into(), id: GUEST_ID }); }

        // DOne
        Ok(res)
    }
}



/// A JSON struct describing a single user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// The ID of the user. Is supposed to be unique.
    #[serde(skip)]
    pub id       : UserId,
    /// The username of the user. Must also be unique.
    pub username : String,
    /// The public certficate(s) file of the user. Will be used to authenticate the connections.
    pub certs    : PathBuf,

    /// The permissions of this user.
    pub permissions : Permissions,
}
