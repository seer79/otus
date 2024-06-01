use libprotocol::{error, server::*};

pub fn main() {
    let server = TcpServer::bind(format!("0.0.0.0:8088")).unwrap();
    server.incoming().for_each(|conn| match conn {
        Ok(mut client) => {
            println!("INFO: connected from {:?}", client.peer_addr().unwrap());
            loop {
                match client.recv_request() {
                    Ok(p) => match client.send_response(p.clone()) {
                        Ok(_) => {
                            println!("Handled packet {:?}", &p);
                        }
                        Err(v) => {
                            println!("ERROR: client send response error {:?}", v);
                        }
                    },
                    Err(error::RecvError::Io(v)) => match v.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            println!("INFO: Client disconnected");
                            return;
                        }
                        _ => {
                            println!("ERROR: Client io error {:?}", v);
                        }
                    },
                    Err(v) => {
                        println!("ERROR: client receive error {:?}", v);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            println!("ERROR: connection error {}", e)
        }
    })
}
