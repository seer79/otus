use std::io::{Read, Write};

pub mod error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Packet {
    Byte(u8),
    Int32(i32),
    Float32(f32),
    Str(String),
}

impl Packet {
    pub fn packet_type(&self) -> u8 {
        match self {
            Packet::Byte(_) => BYTE_PREFIX,
            Packet::Int32(_) => I32_PREFIX,
            Packet::Float32(_) => F32_PREFIX,
            Packet::Str(_) => STR_PREFIX,
        }
    }
}

#[repr(u8)]
enum State {
    Initial,
    ParsePrefix,
    ReadByte,
    ReadI32,
    ReadF32,
    ReadStr,
    Fail,
}

const BYTE_PREFIX: u8 = 0x01;
const I32_PREFIX: u8 = 0x01 << 1;
const F32_PREFIX: u8 = 0x01 << 2;
const STR_PREFIX: u8 = 0x01 << 3;

pub fn read_packet<Reader: Read>(mut reader: Reader) -> Result<Packet, error::RecvError> {
    let mut prefix = [0u8; 1];
    let mut val8 = [0u8; 1];
    let mut val32 = [0u8; 4];
    let mut state = State::Initial;
    loop {
        match state {
            State::Initial => {
                reader.read_exact(&mut prefix)?;
                state = State::ParsePrefix;
            }
            State::Fail => return Err(error::RecvError::InvalidFormat),
            State::ParsePrefix => {
                match prefix[0] {
                    BYTE_PREFIX => {
                        state = State::ReadByte;
                    }
                    F32_PREFIX => {
                        state = State::ReadF32;
                    }
                    I32_PREFIX => {
                        state = State::ReadI32;
                    }
                    STR_PREFIX => {
                        state = State::ReadStr;
                    }
                    _ => {
                        println!("Unexpected format {}", prefix[0]);
                        state = State::Fail;
                    }
                };
            }
            State::ReadByte => {
                reader.read_exact(&mut val8)?;
                return Ok(Packet::Byte(val8[0]));
            }
            State::ReadI32 => {
                reader.read_exact(&mut val32)?;
                return Ok(Packet::Int32(i32::from_be_bytes(val32))); // TCP uses big endian
            }
            State::ReadF32 => {
                reader.read_exact(&mut val32)?;
                return Ok(Packet::Float32(f32::from_be_bytes(val32)));
            }
            State::ReadStr => {
                reader.read_exact(&mut val32)?; // length
                let length = u32::from_be_bytes(val32);
                let mut buff = vec![0u8; length as _];
                reader.read_exact(&mut buff)?;
                let res = String::from_utf8(buff).map_err(|_| error::RecvError::InvalidFormat)?;
                return Ok(Packet::Str(res));
            }
        };
    }
}

pub fn write_packet<Writer: Write>(
    mut writer: Writer,
    packet: Packet,
) -> Result<(), error::SendError> {
    let mut prefix = vec![0u8; 1];
    prefix[0] = packet.packet_type();
    writer.write_all(&prefix)?;

    match packet {
        Packet::Byte(v) => {
            let mut buff = vec![0u8; 1];
            buff[0] = v;
            writer.write_all(&buff)?;
        }
        Packet::Float32(v) => {
            let buff = f32::to_be_bytes(v);
            writer.write_all(&buff)?;
        }
        Packet::Int32(v) => {
            let buff = i32::to_be_bytes(v);
            writer.write_all(&buff)?;
        }
        Packet::Str(s) => {
            let buff = s.as_bytes();
            let len = u32::to_be_bytes(s.len() as u32);
            writer.write_all(&len)?;
            writer.write_all(&buff)?;
        }
        _ => return Err(error::SendError::UnexpectedPacket),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_send() {
        let mut buff = vec![];
        write_packet(&mut buff, Packet::Byte(100)).unwrap();
        write_packet(&mut buff, Packet::Float32(100.2)).unwrap();
        write_packet(&mut buff, Packet::Int32(342)).unwrap();
        write_packet(&mut buff, Packet::Str(String::from("Hello World!"))).unwrap();

        let mut cursor = Cursor::new(buff);
        assert!(matches!(
            read_packet(&mut cursor).unwrap(),
            Packet::Byte(100)
        ));
        assert!(matches!(
            read_packet(&mut cursor).unwrap(),
            Packet::Float32(_)
        ));
        assert!(matches!(
            read_packet(&mut cursor).unwrap(),
            Packet::Int32(342)
        ));
        let spack = read_packet(&mut cursor).unwrap();
        assert!(matches!(&spack, Packet::Str(_),));
        match spack {
            Packet::Str(v) => assert_eq!(v, String::from("Hello World!")),
            _ => panic!("bad packet"),
        }
    }
}
