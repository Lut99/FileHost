/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 20:56:29
 * Last edited:
 *   16 Apr 2022, 16:48:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the FileHost server/
**/

use std::os::unix::net::UnixStream;

use log::{debug, error, info};
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

use filehost_srv::config::Config;


/***** ENTRYPOINT *****/
fn main() {
    // Use the path specified there to read the config
    let config = match Config::new() {
        Ok(config) => config,
        Err(err)   => { eprintln!("Could not load configuration file: {}", err); std::process::exit(1); }  
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
    debug!("Config path: '{}'", config.path.display());



    // Open a connection to the unix socket
    let mut stream = match UnixStream::connect(&config.socket_path) {
        Ok(stream) => stream,
        Err(err)   => { error!("Could not connect to socket '{}': {}", config.socket_path.display(), err); std::process::exit(1); }
    };
}
