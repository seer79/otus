use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{
    error::{self, SendError},
    read_packet, write_packet, Packet,
};

pub struct TcpServer {
    tcp: TcpListener,
}

pub struct TcpConnection {
    stream: TcpStream,
}

impl TcpServer {
    pub fn bind(addr: String) -> Result<TcpServer, error::BindError> {
        let tcp = TcpListener::bind(addr)?;
        Ok(Self { tcp })
    }

    pub fn incoming(
        &self,
    ) -> impl Iterator<Item = Result<TcpConnection, error::ConnectError>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(error::ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> Result<TcpConnection, error::ConnectError> {
        let pack = Packet::Byte(42);
        match write_packet(&mut stream, pack) {
            Err(_) => Err(error::ConnectError::BadHandshake(String::from(
                "Cannot send handshake packet",
            ))),
            Ok(_) => match read_packet(&mut stream) {
                Err(_v) => Err(error::ConnectError::BadHandshake(String::from(
                    "Cannot receive handshake packet",
                ))),
                Ok(Packet::Byte(24)) => Ok(TcpConnection { stream }),
                _ => Err(error::ConnectError::BadHandshake(String::from(
                    "Invalid client",
                ))),
            },
        }
    }
}

impl TcpConnection {
    pub fn send_response_vec(&mut self, response: &[Packet]) -> Result<(), SendError> {
        response
            .iter()
            .try_for_each(|packet| crate::write_packet(&mut self.stream, packet.clone()))
    }

    pub fn send_response(&mut self, response: Packet) -> Result<(), SendError> {
        crate::write_packet(&mut self.stream, response)
    }

    pub fn recv_request(&mut self) -> Result<Packet, error::RecvError> {
        crate::read_packet(&mut self.stream)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
