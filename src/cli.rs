use clap::{Arg, ArgAction, ArgMatches, Command};

const APP_NAME: &str = "udp_p2p";
const APP_VERSION: &str = "v0.1.0";
const AUTHOR: &str = "Abu Zakaria <asm.zakaria120@gmail.com>";

pub fn new() -> ArgMatches {
    return Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(AUTHOR)
        .arg(
            Arg::new("serve")
                .short('s')
                .long("serve")
                .action(ArgAction::SetTrue)
                .help("Create a STUN server."),
        )
        .arg(
            Arg::new("send")
                .long("send")
                .action(ArgAction::Append)
                .help("Send a file."),
        )
        .get_matches();
}
