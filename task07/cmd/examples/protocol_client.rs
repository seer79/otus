use libprotocol::client::{self, *};

pub fn main() {
    let client = client::TcpClient::connect(format!("127.0.0.1:8088"));
    match client {
        Err(v) => panic!("cannot connect to the server {}", v),
        Ok(mut conn) => loop {
            loop {
                println!("command {:?}", conn.send_cmd(123));
            }
        },
    }
}
