use std::net::TcpStream;

use crate::{
    error::{self, CmdError, ConnectError},
    Packet,
};

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn connect(addr: String) -> Result<TcpClient, error::ConnectError> {
        let conn = TcpStream::connect(addr)?;
        Self::try_handshake(conn)
    }

    fn try_handshake(mut stream: TcpStream) -> Result<TcpClient, error::ConnectError> {
        let mut state = 0;
        loop {
            match state {
                0 => {
                    crate::write_packet(&mut stream, crate::Packet::Byte(42))
                        .map_err(|v| error::ConnectError::BadHandshake(v.to_string()))?;
                    state = 1
                }
                1 => match crate::read_packet(&mut stream) {
                    Err(v) => return Err(ConnectError::BadHandshake(v.to_string())),
                    Ok(Packet::Byte(24)) => {
                        state = 3;
                    }
                    _ => state = 4,
                },
                3 => return Ok(TcpClient { stream }),
                4 => {
                    return Err(ConnectError::BadHandshake(String::from(
                        "invalid handshake code",
                    )))
                }
                _ => {
                    return Err(ConnectError::BadHandshake(String::from(
                        "invalid handshake state",
                    )))
                }
            }
        }
    }

    pub fn send_cmd(&mut self, cmd: u8) -> Result<Packet, CmdError> {
        crate::write_packet(&mut self.stream, cmd.into())?;
        crate::read_packet(&mut self.stream).map_err(|v| CmdError::Recv(v))
    }
}
