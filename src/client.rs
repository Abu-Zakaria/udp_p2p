use log::{debug, info};
use std::error::Error;
use std::net::UdpSocket;
use std::{thread, time};

pub struct Client;

const LOCAL_HOST: &str = "0.0.0.0";
const LOCAL_PORT: &str = "4823";

pub const ASK_CODE: &str = "ASK_CODE";
pub const CODE_LENGTH: usize = 4;
pub const BROADCAST: &str = "BROADCAST";

impl Client {
    pub fn connect_stun(stun_address: &str, connect_with: &str) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", LOCAL_HOST, LOCAL_PORT)).unwrap();

        info!("Opened an UDP port on {}:{}", LOCAL_HOST, LOCAL_PORT);

        socket.connect(stun_address)?;

        info!("Connected to the STUN server [{stun_address}]");

        if connect_with != "" {
            let remote_address = Self::ask_with_code(&socket, &connect_with).unwrap();

            info!("Received remote client's ip address");
            debug!("Remote client: [{remote_address}]");

            Self::broadcast_ip(&socket, &connect_with)?;

            loop {
                debug!("Trying to connect with remote peer");
                Self::connect_remote(&socket, &remote_address)?;

                let sleep_delay = time::Duration::from_millis(500);
                thread::sleep(sleep_delay);
            }

            // Keep connecting to the remote client
            // and the remote client also needs to try keep connecting to local client
        } else {
            Self::register(&socket)?;
            info!("Registered to STUN server");

            let mut buf = [0; CODE_LENGTH];

            let length = socket.recv(&mut buf)?;

            if length != CODE_LENGTH {
                Err("Code length from STUN server is invalid!")?
            }

            let code = String::from_utf8(buf.to_vec())?;

            info!("Your code is: {code}");

            loop {
                let mut buf = [0; 32];
                let length = socket.recv(&mut buf)?;
                info!(
                    "Received string: {}",
                    String::from_utf8(buf[0..length].to_vec())?
                )
            }
        }

        //         loop {
        //             let mut buf = [0; 5];
        //             let length = socket.recv(&mut buf)?;

        //             info!(
        //                 "Received length bytes -> {}",
        //                 String::from_utf8(buf[0..length].to_vec())?
        //             );
        //         }
    }

    pub fn register(socket: &UdpSocket) -> Result<(), Box<dyn Error>> {
        let message = String::from("REGISTER");

        let length = socket.send(message.as_bytes())?;

        if length == 0 {
            Err("Something went wrong while sending REGISTER message to STUN server")?
        }

        Ok(())
    }

    fn ask_with_code(socket: &UdpSocket, code: &str) -> Result<String, Box<dyn Error>> {
        let mut buf = String::from(ASK_CODE);

        buf = buf + ":" + code;

        match socket.send(&buf.as_bytes()) {
            Ok(length) => {
                if length == 0 {
                    Err("Failed to ask for the code")?
                }
            }
            Err(err) => Err(err)?,
        };

        let mut buf = [0; 64];

        match socket.recv(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    Ok(String::from_utf8(buf.to_vec())?)
                } else {
                    Err("Responsing from STUN is not valid!")?
                }
            }
            Err(error) => Err(error)?,
        }
    }

    fn broadcast_ip(socket: &UdpSocket, code: &str) -> Result<(), Box<dyn Error>> {
        let mut message = String::from(BROADCAST);

        message += code;

        match socket.send(message.as_bytes()) {
            Ok(size) => {
                if size != 13 {
                    Err("Couldn't broadcast successfully!")?
                }
            }
            Err(error) => Err(error)?,
        }
        Ok(())
    }

    fn connect_remote(socket: &UdpSocket, remote_address: &str) -> Result<(), Box<dyn Error>> {
        socket.connect(remote_address)?;

        info!("Connected with remote client");

        let message = String::from("Hi there");

        socket.send(message.as_bytes())?;

        Ok(())
    }
}
