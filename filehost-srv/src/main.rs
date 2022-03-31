/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:56:29
 * Last edited:
 *   31 Mar 2022, 18:28:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the FileHost server/
**/

use clap::Parser;
use log::info;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

use filehost_srv::config::{Arguments, Config};


/***** ENTRYPOINT *****/
fn main() {
    // Get the arguments
    let args = Arguments::parse();
    // Use the path specified there to read the config
    let config = match Config::from_path(&args.config_path) {
        Ok(config) => config,
        Err(err)   => { eprintln!("Could not load configuration file '{}': {}", args.config_path.display(), err); std::process::exit(1); }  
    };

    // Prepare the logger(s)
    if connected_to_journal() {
        // We are connected to systemd, so create that logger
        init_with_extra_fields(vec![
            ("VERSION", env!("CARGO_PKG_VERSION"))
        ]).unwrap_or_else(|err| panic!("Could not create systemd logger: {}", err));
        log::set_max_level(config.log_level);
    } else {
        // We are not connected, so just use the terminal logger
        TermLogger::init(config.log_level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto)
            .unwrap_or_else(|err| panic!("Could not create stderr logger: {}", err));
    }
    info!("Initializing FileHost Server v{}", env!("CARGO_PKG_VERSION"));



    // Open a connection to the unix socket
    
}
