/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:38:01
 * Last edited:
 *   16 Apr 2022, 16:41:16
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Module that handles the command-line interface part of the ctl.
**/

use clap::Parser;


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
    
}
