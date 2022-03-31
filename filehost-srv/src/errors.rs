/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:34:34
 * Last edited:
 *   31 Mar 2022, 18:10:47
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
/// Errors that relate to the Config struct in config.rs.
#[derive(Debug)]
pub enum ConfigError {
    /// Could not open the configuration file
    OpenError{ path: PathBuf, err: std::io::Error },
    /// Could not parse the file as JSON
    ParseError{ path: PathBuf, err: serde_json::Error },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            ConfigError::OpenError{ path, err }  => write!(f, "Could not open configuration file '{}': {}", path.display(), err),
            ConfigError::ParseError{ path, err } => write!(f, "Could not parse configuration file '{}': {}", path.display(), err),
        }
    }
}

impl Error for ConfigError {}
