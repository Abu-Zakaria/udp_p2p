use log::{debug, error, info};
use std::error;
use std::net::UdpSocket;

pub struct StunServer<'a> {
    pub host: &'a str,
    pub port: &'a str,
}

impl<'a> StunServer<'a> {
    pub fn start(self: &Self) -> Result<(), Box<dyn error::Error>> {
        let result = UdpSocket::bind(format!("{}:{}", self.host, self.port));

        let socket = result?;

        debug!("Started STUN server successfully at: {:#?}", socket);

        let mut buf = [0; 128];

        match socket.recv_from(&mut buf) {
            Ok((length, addr)) => {
                info!("Received {length} bytes from {addr}");
                info!(
                    "Message: {}",
                    String::from_utf8(buf[..length].to_vec()).unwrap()
                );
            }
            Err(error) => {
                error!("Something went wrong while receiving bytes from a remote source.");
                debug!("The error message: {error}");
            }
        }

        Ok(())
    }
}
