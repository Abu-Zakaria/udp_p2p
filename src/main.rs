use udp_p2p::cli;

fn main() {
    let matches = cli::new();

    if let Some(_serve_flag) = matches.get_one::<bool>("serve") {
        todo!("start udp STUN server");
    }

    if let Some(_send_flag) = matches.get_one::<String>("send") {
        todo!("send the data");
    }
}
