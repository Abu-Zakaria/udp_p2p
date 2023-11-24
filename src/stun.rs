use crate::client;
use crate::utils::ip_address;
use log::{debug, error, info};
use rand::{distributions::Alphanumeric, Rng};
use std::error;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
use std::vec::Vec;

pub const REQUEST: &str = "REQUEST";

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
                            error!("Couldn't convert the incoming bytes into UTF-8 string");
                            debug!("ERROR: {}", error);
                        }
                    }

                    if incoming_message_str == "REGISTER" {
                        info!("Registering this IP address: {addr}");

                        let (is_registered, code) = self.register(addr);

                        if is_registered {
                            info!("Successfully registered IP: {}", addr.ip());
                            info!("Code: {code}");
                        }

                        let length = socket.send_to(code.as_bytes(), addr)?;

                        if length != client::CODE_LENGTH {
                            error!("Code was not sent back successfully. IP: {addr}");
                        } else {
                            info!("Code sent to: {addr}")
                        }
                    } else if &incoming_message_str[0..client::ASK_CODE.len()] == client::ASK_CODE {
                        match self.response_with_code(
                            &socket,
                            addr,
                            &incoming_message_str[(client::ASK_CODE.len() + 1)..],
                        ) {
                            Ok(remote_ip) => {
                                info!("Responded with their requested code's ip address [{remote_ip}]");
                            }
                            Err(error) => {
                                info!("Couldn't respond to a remote ip with their requested code's ip address");
                                Err(error)?
                            }
                        }
                    } else if &incoming_message_str[0..client::BROADCAST.len()] == client::BROADCAST
                    {
                        let remote_host = addr.ip();
                        let remote_port = addr.port();
                        let code = &incoming_message_str[client::BROADCAST.len()..];

                        debug!("code: {:?}", code);

                        let addresses = &self.registered_ipv4_addresses;

                        if let Some(item) = addresses.into_iter().find(|&item| item.code == code) {
                            println!("matched with {} {}", item.code, item.addr.ip());

                            // send the remote client's address to the matched ip address
                            // so they can establish a p2p connection
                            self.inform_other_peer(
                                &socket,
                                item.addr.ip().to_string() + ":" + &item.addr.port().to_string(),
                                remote_host.to_string() + ":" + &remote_port.to_string(),
                            );
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
            (false, item.code.clone())
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
            .take(client::CODE_LENGTH)
            .map(char::from)
            .collect()
    }

    fn response_with_code(
        &mut self,
        socket: &UdpSocket,
        addr: SocketAddr,
        code: &str,
    ) -> Result<String, Box<dyn error::Error>> {
        let code = String::from(code.trim());

        let addresses = &self.registered_ipv4_addresses;

        if let Some(registered_ip) = addresses.into_iter().find(|&x| x.code == code) {
            let remote_host = ip_address::octets_to_string(registered_ip.addr.ip().octets());
            let remote_ip: String =
                remote_host + &String::from(":") + &registered_ip.addr.port().to_string();

            match socket.send_to(remote_ip.as_bytes(), addr) {
                Ok(length) => {
                    if length == 0 {
                        Err("Couldn't send back the requested ")?
                    } else {
                        return Ok(remote_ip);
                    }
                }
                Err(error) => Err(error)?,
            }
        }

        Err("No registered client found with the code")?
    }

    fn inform_other_peer(self: &Self, socket: &UdpSocket, peer: String, remote_address: String) {
        debug!("peer: {peer}");
        debug!("remote: {remote_address}");

        let message = String::from(REQUEST) + &remote_address;

        match socket.send_to(message.as_bytes(), peer) {
            Ok(length) => {
                debug!("sent data's length: {length}")
            }
            Err(error) => {
                error!("found an error: {error}")
            }
        }
    }
}

pub struct RegisteredIpv4Address {
    addr: SocketAddrV4,
    code: String,
}
