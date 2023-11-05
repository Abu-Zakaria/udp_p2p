use log;
use log::{debug, error, info, LevelFilter};
use udp_p2p::cli;
use udp_p2p::client::Client;
use udp_p2p::logger;
use udp_p2p::stun::new as new_stun_server;

const STUN_HOST: &str = "0.0.0.0";
const STUN_PORT: &str = "6969";

fn main() {
    static LOGGER: logger::Logger = logger::Logger {
        level: LevelFilter::Debug,
    };

    if let Err(error) = log::set_logger(&LOGGER) {
        eprintln!("Couldn't set custom logger");
        eprintln!("Error message: {error}");
    }

    log::set_max_level(LOGGER.level);

    let matches = cli::new();

    if let Some(serve_flag) = matches.get_one::<bool>("serve") {
        if *serve_flag {
            let mut stun_server = new_stun_server(STUN_HOST, STUN_PORT);

            match stun_server.start() {
                Ok(()) => info!("Closing the STUN server!"),
                Err(error) => {
                    error!("Something went wrong with the stun server");
                    debug!("The error message: {error}");
                }
            }
        }
    }

    let mut stun_flag = String::from("");
    let mut connect_flag = String::from("");

    match matches.get_one::<String>("stun") {
        Some(flag) => stun_flag = flag.to_string(),
        None => debug!("STUN server address not given"),
    }

    match matches.get_one::<String>("connect code") {
        Some(flag) => connect_flag = flag.to_string(),
        None => debug!("Connect code not given"),
    }

    match Client::connect_stun(&stun_flag, &connect_flag) {
        Ok(()) => info!("Completed all actions. Goodbye!"),
        Err(error) => {
            error!("Something went wrong while registering to STUN server");
            debug!("The error message: {error}");
        }
    }
}
