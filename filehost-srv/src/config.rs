/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:57:06
 * Last edited:
 *   16 Apr 2022, 13:16:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the CLI-parser for the daemon.
**/

use std::fs::File;
use std::path::{Path, PathBuf};

use clap::Parser;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use filehost_spc::consts::DEFAULT_CONFIG_PATH;

use crate::errors::ConfigError as Error;


/***** ARGUMENTS *****/
/// Defines the toplevel command-line interface by using clap's derive API.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// The path to the config file.
    #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The path to the config file.")]
    pub config_path : PathBuf,
}





/***** CONFIG *****/
/// Defines the configuration file, serialized & deserialized with serde.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The log level to apply.
    pub log_level : LevelFilter,

    /// The socket path to listen for.
    pub socket_path : PathBuf,

    /// The address:port to listen on.
    pub listen_addr : String,
}

impl Config {
    /// Constructor for the Config.  
    /// Will read the config file to load from the CLI.
    /// 
    /// # Returns
    /// The new Config instance on success, or an Error otherwise.
    pub fn new() -> Result<Self, Error> {
        // Parse the CLI
        let args = Arguments::parse();

        // Let from_path() for the rest
        Self::from_path(&args.config_path)
    }

    /// Constructor for the Config, which reads the file from the given path on the disk.
    /// 
    /// # Generic types
    ///  - `P`: The Path-like type of the configuration file location.
    /// 
    /// # Arguments
    ///  - `path`: The path to the configuration file to read.
    /// 
    /// # Returns
    /// The new Config instance on success, or an Error otherwise.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Convert the path-like to a path
        let path = path.as_ref();

        // Open the file
        let handle = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::OpenError{ path: path.to_path_buf(), err }); }
        };

        // Parse with serde
        match serde_json::from_reader(handle) {
            Ok(config) => Ok(config),
            Err(err)   => Err(Error::ParseError{ path: path.to_path_buf(), err }),
        }
    }
}
