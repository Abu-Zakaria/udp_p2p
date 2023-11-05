use log::info;
use std::error::Error;
use std::net::UdpSocket;

pub struct Client;

const LOCAL_HOST: &str = "0.0.0.0";
const LOCAL_PORT: &str = "4823";
const ASK_CODE: &str = "ASK_CODE";

pub const CODE_LENGTH: usize = 4;

impl Client {
    pub fn connect_stun(stun_address: &str, connect_with: &str) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", LOCAL_HOST, LOCAL_PORT)).unwrap();

        info!("Opened an UDP port on {}:{}", LOCAL_HOST, LOCAL_PORT);

        socket.connect(stun_address)?;

        info!("Connected to the STUN server [{stun_address}]");

        if connect_with != "" {
            let remote_address = Self::ask_with_code(&socket, &connect_with)?;

            info!("Successfully connected with remote address");
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
        }

        Ok(())
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
}
