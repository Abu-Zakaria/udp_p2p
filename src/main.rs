use log;
use log::{debug, error, info, LevelFilter};
use udp_p2p::cli;
use udp_p2p::logger;
use udp_p2p::stun::StunServer;

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
            let stun_server = StunServer {
                host: STUN_HOST,
                port: STUN_PORT,
            };

            match stun_server.start() {
                Ok(()) => info!("Closing the STUN server!"),
                Err(error) => {
                    error!("Something went wrong with the stun server");
                    debug!("The error message: {error}");
                }
            }
        }
    }

    if let Some(_stun_flag) = matches.get_one::<String>("stun") {
        // todo!("make connection to STUN server");
    }

    if let Some(_connect_flag) = matches.get_one::<String>("connect code") {
        // todo!("connect with someone with a code");
    }
}
