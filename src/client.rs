use log::info;
use std::error::Error;
use std::net::UdpSocket;

pub struct Client;

static LOCAL_HOST: &str = "0.0.0.0";
static LOCAL_PORT: &str = "4823";

impl Client {
    pub fn connect(stun_address: &str) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", LOCAL_HOST, LOCAL_PORT)).unwrap();

        info!("Opened an UDP port on {}:{}", LOCAL_HOST, LOCAL_PORT);

        socket.connect(stun_address)?;

        info!("Connected to the STUN server");

        Self::register(socket)?;

        info!("Registered to STUN server");

        Ok(())
    }

    pub fn register(socket: UdpSocket) -> Result<(), Box<dyn Error>> {
        let message = String::from("REGISTER");

        let length = socket.send(message.as_bytes())?;

        if length == 0 {
            Err("Something went wrong while sending REGISTER message to STUN server")?
        }

        Ok(())
    }
}
