/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:38:01
 * Last edited:
 *   01 Apr 2022, 12:06:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Module that handles the command-line interface part of the ctl.
**/

use std::path::PathBuf;

use clap::Parser;

use filehost_spc::consts::{DEFAULT_CONFIG_PATH, DEFAULT_SOCKET_PATH};


/***** CONSTANTS *****/
// Lazy constants
lazy_static! {
    /// The standard user database directory
    static ref DEFAULT_USERBASE_FILE: String = dirs_2::home_dir().expect("Could not get home directory of user").join(".filehost/users.json").to_str().expect("Could not convert user database path to a string").to_string();
    /// The standard database directory
    static ref DEFAULT_DATABASE_DIR: String = dirs_2::home_dir().expect("Could not get home directory of user").join(".filehost/database/").to_str().expect("Could not convert database path to a string").to_string();
}





/***** ARGUMENTS *****/
/// Defines the toplevel command-line interface by using clap's derive API.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// If given, displays logs messages to stdout/stderr
    #[clap(long, help = "If given, displays logs messages to stdout and stderr.")]
    pub debug : bool,

    /// The action to take from this point on (subcommand)
    #[clap(subcommand)]
    pub action : Action,
}



/// Defines the actions / subcommands that can be done on the server.
#[derive(Parser)]
pub enum Action {
    /// Sets the project up, by downloading the server executable and generating files.
    #[clap(name = "install", about = "Prepares the server-side by generating the appropriate files and adding the server as a daemon.")]
    Install {
        #[clap(short, long, help = "If given, does not download the latest version from GitHub but instead uses the given server exeuctable.")]
        server_exec : Option<PathBuf>,

        #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The location of the server's configuration file. Note, though, that if you use anything but the default, you will have to specify the location every time you run the server or the CTL via --config-path.")]
        config_path : PathBuf,
        #[clap(short, long, default_value = &DEFAULT_SOCKET_PATH, help = "The location of the socket path that the CTL uses to communicate with the server.")]
        socket_path : PathBuf,
    },

    /// Tears the project down, by removing the server executable and associated files.
    #[clap(name = "uninstall", about = "Removes the server installation by removing the server as a daemon and deleting all of its configs.")]
    Uninstall {
        #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The config path that contains the server's configuration. You should only use the non-default path if you changed it during installation.")]
        config_path : PathBuf,
    },
}
