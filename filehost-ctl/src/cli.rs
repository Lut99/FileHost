/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:38:01
 * Last edited:
 *   30 Mar 2022, 20:57:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Module that handles the command-line interface part of the ctl.
**/

use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;


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
    /// If given, never writes to systemd/stderr but to the given file instead.
    #[clap(short, long, help = "If given, does not log to systemd/stderr but to the given file instead.")]
    pub log_file  : Option<PathBuf>,
    /// The logging severity to display.
    #[clap(short = 'v', long = "verbosity", default_value = LevelFilter::Debug.as_str(), help = "The log severity to log.")]
    pub log_level : LevelFilter,

    /// The action to take from this point on (subcommand)
    #[clap(subcommand)]
    pub action : Action,
}



/// Defines the actions / subcommands that can be done on the server.
#[derive(Parser)]
pub enum Action {
    /// Generates an empty user database.
    #[clap(name = "generate", about = "Generates a new user database at the given location.")]
    Generate {
        /// The path to the userbase to generate.
        #[clap(short, long, default_value = &DEFAULT_USERBASE_FILE, help = "The location of the new user database file to generate.")]
        userbase : PathBuf,
    },
    /// Generates an empty file database.
    #[clap(name = "init")]
    Initialize {
        /// The path to the userbase to generate.
        #[clap(default_value = &DEFAULT_DATABASE_DIR, help = "The location of the new database directory to generate.")]
        database : PathBuf,
    },

    /// Adds a new user to the given userbase file.
}
