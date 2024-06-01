use std::{sync::Mutex, thread};

use libprotocol::{error::ConnectError, server::TcpConnection};

use crate::{ACSocket, Commands};

/// Run IoT server on specified address and with specified devices
pub fn run_iot_server(addr: String, devs: &Vec<ACSocket>) {
    match libprotocol::server::TcpServer::bind(addr) {
        Err(v) => panic!("cannot start server {}", v),
        Ok(server) => server.incoming().for_each(|item| match item {
            Ok(connection) => {
                let clone = devs.clone();
                thread::spawn(move || handle_connection(connection, clone));
            }
            Err(v) => {
                println!("ERROR: Invalid connection {:?}", v);
            }
        }),
    }
}

fn handle_connection(
    mut connection: TcpConnection,
    devices: Vec<ACSocket>,
) -> Result<(), libprotocol::error::RecvError> {
    enum State {
        IDLE,
        READ_ID,
        HANDLE_CMD,
        SEND_RESULT,
    }
    let mut state = State::IDLE;
    let mut cmd: Option<Commands> = Option::None;
    loop {
        match state {
            State::IDLE => {
                let request = connection.recv_request()?;
                state = State::READ_ID;
                match request {
                    libprotocol::Packet::Byte(v) => {
                        match v {
                            v if v == Commands::GetConsumption as u8 => {
                                cmd = Option::Some(Commands::GetConsumption);
                            }
                            v if v == Commands::GetStatus as u8 => {
                                cmd = Option::Some(Commands::GetStatus);
                            }
                            v if v == Commands::ListDevices as u8 => {
                                cmd = Option::Some(Commands::ListDevices);
                                state = State::HANDLE_CMD;
                            }
                            v if v == Commands::PowerOn as u8 => {
                                cmd = Option::Some(Commands::PowerOn);
                            }
                            v if v == Commands::PowerOff as u8 => {
                                cmd = Option::Some(Commands::PowerOff);
                            }
                            _ => {
                                println!("ERROR: Unsupported command {}", v);
                                state = State::IDLE;
                            }
                        };
                    }
                    _ => {
                        println!("ERROR: Unsupported package {:?}", request);
                    }
                }
            }
            State::READ_ID => {
                state = State::HANDLE_CMD;
            }
            State::HANDLE_CMD => {
                state = State::SEND_RESULT;
            }
            State::SEND_RESULT => {
                state = State::IDLE;
            }
        }
    }
}
