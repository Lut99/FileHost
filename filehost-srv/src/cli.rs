/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:57:06
 * Last edited:
 *   30 Mar 2022, 21:01:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the CLI-parser for the daemon.
**/

use std::path::PathBuf;

use clap::Parser;


/***** CONSTANTS *****/
/// The default config path
const DEFAULT_CONFIG_PATH: &str = "/etc/filehost/config.json";

/// The default socket path
const DEFAULT_SOCKET_PATH: &str = "/var/run/filehost.sock";
/// The default address to listen on
const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:7391";





/***** ARGUMENTS *****/
/// Defines the toplevel command-line interface by using clap's derive API.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// The path to the config file.
    #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The path to the config file.")]
    pub config_path : PathBuf,
}
