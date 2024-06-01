use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{
    error::{self, ConnectError, SendError},
    Packet,
};

#[derive(Debug)]
pub struct TcpServer {
    tcp: TcpListener,
}

#[derive(Debug)]
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
        println!(
            "INFO: Starting server on {:?}",
            self.tcp.local_addr().unwrap()
        );
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(error::ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> Result<TcpConnection, error::ConnectError> {
        println!(
            "INFO: server is trying handshake with {:?}",
            stream.peer_addr().unwrap()
        );
        let mut state = 0;
        loop {
            match state {
                0 => match crate::read_packet(&mut stream) {
                    Ok(Packet::Byte(42)) => state = 1,
                    Err(v) => {
                        return Err(ConnectError::BadHandshake(format!("invalid code {:?}", v,)))
                    }
                    _ => return Err(ConnectError::BadHandshake(String::from("invalid packet"))),
                },
                1 => match crate::write_packet(&mut stream, Packet::Byte(24)) {
                    Ok(_) => {
                        state = 3;
                    }
                    Err(v) => {
                        return Err(ConnectError::BadHandshake(format!(
                            "cannot send response {:?}",
                            v
                        )))
                    }
                },
                3 => return Ok(TcpConnection { stream }),
                _ => return Err(ConnectError::BadHandshake(String::from("invalid state"))),
            }
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
