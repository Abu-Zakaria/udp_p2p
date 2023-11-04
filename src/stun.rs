use log::{debug, error, info};
use rand::{distributions::Alphanumeric, Rng};
use std::error;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
use std::vec::Vec;

pub struct StunServer<'a> {
    pub host: &'a str,
    pub port: &'a str,

    pub registered_ipv4_addresses: Vec<RegisteredIpv4Address>,
}

pub fn new<'a>(host: &'a str, port: &'a str) -> StunServer<'a> {
    StunServer {
        host,
        port,
        registered_ipv4_addresses: vec![],
    }
}

impl<'a> StunServer<'a> {
    pub fn start(&mut self) -> Result<(), Box<dyn error::Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", self.host, self.port))?;

        debug!("Started STUN server successfully at: {:#?}", socket);

        loop {
            let mut buf = [0; 128];

            match socket.recv_from(&mut buf) {
                Ok((length, addr)) => {
                    info!("Received {length} bytes from {addr}");

                    let mut incoming_message_str: String = String::from("");

                    match String::from_utf8(buf[..length].to_vec()) {
                        Ok(message_str) => {
                            incoming_message_str = message_str;
                        }
                        Err(error) => {
                            debug!("Couldn't convert the incoming bytes into UTF-8 string");
                            debug!("ERROR: {}", error);
                        }
                    }

                    if length == 8 && incoming_message_str == "REGISTER" {
                        debug!("Registering this IP address: {addr}");

                        let (is_registered, code) = self.register(addr);

                        if is_registered {
                            debug!("Successfully registered IP: {}", addr.ip());
                            debug!("Code: {code}");
                        }
                    }
                }
                Err(error) => {
                    error!("Something went wrong while receiving bytes from a remote source.");
                    debug!("The error message: {error}");
                    break Err("Stopped the STUN server with an error".into());
                }
            }
        }
    }

    pub fn register(&mut self, address: SocketAddr) -> (bool, String) {
        match address {
            SocketAddr::V4(addr_v4) => self.register_ipv4(addr_v4),
            SocketAddr::V6(_addr_v6) => {
                // TODO: Add support IPv6
                (false, "".to_string())
            }
        }
    }

    fn register_ipv4(&mut self, addr_v4: SocketAddrV4) -> (bool, String) {
        let code = self.generate_code();

        let addresses = &self.registered_ipv4_addresses;

        if let Some(item) = addresses
            .into_iter()
            .find(|&item| return item.addr.ip() == addr_v4.ip())
        {
            debug!("IPv4 address already registered: {}", item.addr.ip());
            (false, String::from(""))
        } else {
            let new_item = RegisteredIpv4Address {
                addr: addr_v4,
                code: String::from(&code),
            };

            self.registered_ipv4_addresses.push(new_item);
            (true, code)
        }
    }

    fn generate_code(&mut self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect()
    }
}

pub struct RegisteredIpv4Address {
    addr: SocketAddrV4,
    code: String,
}
