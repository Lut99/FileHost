/* CONSTS.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 11:39:13
 * Last edited:
 *   01 Apr 2022, 11:53:46
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the constants that are shared across the FileHost project.
**/

use log::LevelFilter;


/***** CONSTANTS *****/
/// The default config path
pub const DEFAULT_CONFIG_PATH: &str = "/etc/filehost/config.json";
/// The default socket path
pub const DEFAULT_SOCKET_PATH: &str = "/var/run/filehost.sock";

/// The default log level to apply
pub const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Debug;
/// The default address to listen on
pub const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:7391";
