//  MAIN.rs
//    by Lut99
// 
//  Created:
//    21 Aug 2022, 13:16:40
//  Last edited:
//    28 Aug 2022, 23:01:06
//  Auto updated?
//    Yes
// 
//  Description:
//!   Simple proxy service that handles SSL and forwards requests to other
//!   services.
// 

use std::fs::File;
use std::net::Ipv4Addr;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::Parser;
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, SharedLogger, TerminalMode, TermLogger, WriteLogger};
use warp::Filter;
use warp_reverse_proxy::reverse_proxy_filter;


/***** HELPER FUNCTIONS *****/
/// Resolves the given logging path by filling in '$DATE' and '$TIME'.
/// 
/// # Generic arguments
/// - `S`: The &str-like type of the `log_path`.
/// 
/// # Arguments
/// - `log_path`: The path to resolve.
/// 
/// # Returns
/// The same string as a PathBuf, now with `$DATE` replaced with the current date and `$TIME` replaced with the current time.
fn resolve_log_path<S: AsRef<str>>(log_path: S) -> PathBuf {
    let log_path: &str = log_path.as_ref();

    // Get the current time
    let now: DateTime<Local> = Local::now();

    // Replace the date, then the time
    let result: String = log_path.replace("$DATE", &now.format("%Y-%m-%d").to_string());
    let result: String = result.replace("$TIME", &now.format("%H-%M-%S").to_string());

    // Dome
    PathBuf::from(result)
}





/***** ARGUMENTS *****/
/// Defines the command-line (and environment) variabels for the proxy service.
#[derive(Parser)]
struct Args {
    /// Defines the path of the logfile. Can use '$DATE'and '$TIME' to be replaced with up-to-date values.
    #[clap(short, long, default_value="/logs/filehost-prx_$DATE_$TIME.log", help="The path of the logfile. Can use '$DATE'and '$TIME' to be replaced with up-to-date values.", env="LOG_PATH")]
    log_path : String,

    /// The IP-address to listen on. Use '0.0.0.0' to open it to the outside world.
    #[clap(short, long, default_value="127.0.0.1", help="The IP-address to serve on.", env="IP_ADDRESS")]
    ip_address : Ipv4Addr,
    /// The port to listen on.
    #[clap(short, long, default_value="8719", help="The port to serve on.", env="PORT")]
    port       : u16,

    /// Defines the endpoint to direct download/upload requests to.
    #[clap(short, long, help="The endpoint of the file service.", env="FILE_ENDPOINT")]
    file_endpoint     : String,
    /// Defines the endpoint to direct registry requests to.
    #[clap(short, long, help="The endpoint of the registry service.", env="REGISTRY_ENDPOINT")]
    registry_endpoint : String,
    /// Defines the endpoint to direct authentication requests to.
    #[clap(short, long, help="The endpoint of the authentication service.", env="AUTH_ENDPOINT")]
    auth_endpoint     : String,
}





/***** LIBRARY *****/
/// Entrypoint to the binary.
#[tokio::main]
async fn main() {
    // Read command-line arguments
    let args: Args = Args::parse();

    // Setup the logger(s)
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![ TermLogger::new(LevelFilter::Debug, Default::default(), TerminalMode::Mixed, ColorChoice::Auto) ];
    let log_path: PathBuf = resolve_log_path(&args.log_path);
    match File::create(&log_path) {
        Ok(handle) => {
            loggers.push(WriteLogger::new(LevelFilter::Debug, Default::default(), handle));
        },
        Err(err) => {
            eprintln!("WARNING: Could not create log file '{}': {}", log_path.display(), err);
            eprintln!("WARNING: No log file will be generated for this session.");
        }
    }
    if let Err(err) = CombinedLogger::init(loggers) {
        eprintln!("WARNING: Could not initialize logger(s): {}", err);
        eprintln!("WARNING: No further logs will be printed for this session.");
    }
    info!("Initializing FileHost Proxy service v{}...", env!("CARGO_PKG_VERSION"));

    // Setup the proxy paths
    let file = warp::path!("file" / ..).and(
        reverse_proxy_filter("file/".into(), args.file_endpoint.clone())
    );
    let reg = warp::path!("registry" / ..).and(
        reverse_proxy_filter("registry/".into(), args.registry_endpoint.clone())
    );
    let auth = warp::path!("auth" / ..).and(
        reverse_proxy_filter("auth/".into(), args.auth_endpoint.clone())
    );

    // Run the server with the given app
    warp::serve(file.or(reg.or(auth)).with(warp::log("filehost-prx"))).tls().run((args.ip_address, args.port)).await;
}
