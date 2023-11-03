use clap::{Arg, ArgAction, ArgMatches, Command};

const APP_NAME: &str = "udp_p2p";
const APP_DESCRIPTION: &str = "This cli tool makes a direct UDP connection between 2 devices using a STUN server. After connection between the devices has established for first time, the devices will keep communicating to each other without using the server. It will be a direct client-to-client connection.";
const APP_VERSION: &str = "v0.1.0";
const AUTHOR: &str = "Abu Zakaria <asm.zakaria120@gmail.com>";

pub fn new() -> ArgMatches {
    return Command::new(APP_NAME)
        .about(APP_DESCRIPTION)
        .version(APP_VERSION)
        .author(AUTHOR)
        .arg_required_else_help(true)
        .arg(
            Arg::new("serve")
                .short('s')
                .long("serve")
                .action(ArgAction::SetTrue)
                .help("Create a STUN server."),
        )
        .arg(
            Arg::new("stun")
                .long("stun")
                .action(ArgAction::Append)
                .help("STUN server address [X.X.X.X:YYYY]"),
        )
        .arg(
            Arg::new("connect code")
                .long("connect")
                .action(ArgAction::Append)
                .help(
                    "The code of the device you want to connect with.
If you dont have any code, you can run the command without this flag and you will get a code.
Share the code with someone you want to connect with.",
                ),
        )
        .get_matches();
}
