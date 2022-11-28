use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub fn read_byte(mut stream: &TcpStream) -> u8
{
    let mut buf: Vec<u8> = vec![0;1];

    match stream.read_exact(&mut buf)
    {
        Ok(_)=> {

        },
        Err(e) => {
            println!("[-] Failed to read byte from stream - {:?} -", e);
            return 0;
        }
    }

    return buf[0];
}

pub fn read_long(mut stream: &TcpStream) -> u64
{
    let mut buf: Vec<u8> = vec![0;8];

    match stream.read_exact(&mut buf)
    {
        Ok(_)=> {

        },
        Err(e) => {
            println!("[-] Failed to read int from stream - {:?} -", e);
            return 0;
        }
    }

    let mut num = Cursor::new(buf);

    return num.read_u64::<BigEndian>().unwrap();
}