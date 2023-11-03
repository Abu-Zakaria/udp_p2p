use udp_p2p::cli;

fn main() {
    let matches = cli::new();

    if let Some(_serve_flag) = matches.get_one::<bool>("serve") {
        // todo!("start udp STUN server");
    }

    if let Some(_stun_flag) = matches.get_one::<String>("stun") {
        // todo!("make connection to STUN server");
    }

    if let Some(_connect_flag) = matches.get_one::<String>("connect code") {
        // todo!("connect with someone with a code");
    }
}
