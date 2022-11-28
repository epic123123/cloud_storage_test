use std::{net::TcpStream, io::Write};
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

pub fn write(mut stream: &TcpStream, filename: String, mut data: Vec<u8>)
{
    let mut buf: Vec<u8> = vec![];
    buf.push(1);
    buf.append(&mut filename.len().to_be_bytes().to_vec());
    buf.append(&mut filename.as_bytes().to_vec());
    buf.append(&mut data.len().to_be_bytes().to_vec());
    buf.append(&mut data);

    if let Err(e) = stream.write_all(&mut buf)
    {
        println!("[-] Failed to write to stream on write - {:?} -", e);
        return;
    }

    // response

    let packet_id = read_byte(&mut stream);

    if packet_id != 1
    {
        println!("[-] Server responded with an invalid packet id on write");
        return;
    }

    let success = read_byte(&mut stream);

    if success != 1
    {
        println!("[-] Server ran into an error while trying to write file");
        return;
    }
}

pub fn read(mut stream: &TcpStream, filename: String) -> Result<Vec<u8>, &'static str>
{
    let mut buf: Vec<u8> = vec![];

    buf.push(2_u8);
    buf.append(&mut filename.len().to_be_bytes().to_vec());
    buf.append(&mut filename.as_bytes().to_vec());

    if let Err(e) = stream.write_all(&mut buf)
    {
        println!("[-] Failed to write to stream on write - {:?} -", e);
        return Err("Error");
    }

    let packet_id = read_byte(&mut stream);

    if packet_id != 2
    {
        println!("[-] Server responded with an invalid packet id on read");
        return Err("Error");
    }

    let success = read_byte(&mut stream);

    if success != 1
    {
        println!("[-] Server ran into an error while trying to read file");
        return Err("Error");
    }

    let data_len = read_long(&mut stream);

    let mut data: Vec<u8> = vec![0; data_len as usize];

    if let Err(e) = stream.read_exact(&mut data)
    {
        println!("[-] Server ran into an error while trying to read file - {:?} -", e);
        return Err("Error");
    }


    return Ok(data);
}

pub fn filenames(mut stream: &TcpStream) -> Result<Vec<String>, &'static str>
{
    if let Err(e) = stream.write(&[3])
    {
        println!("[-] Failed to write to stream on filename retrieval - {:?} -", e);
        return Err("Error");
    }

    let packet_id = read_byte(&mut stream);

    if packet_id != 3
    {
        println!("[-] Server responded with an invalid packet id on filename retrieval");
        return Err("Error");
    }

    let len = read_long(&mut stream);

    let mut n: Vec<u8> = vec![0; len as usize];

    if let Err(e) = stream.read_exact(&mut n)
    {
        println!("[-] Server ran into an error while trying to read filenames - {:?} -", e);
        return Err("Error");
    }

    let names = n.split(|c| c == &('\0' as u8));

    let mut names_vec: Vec<String> = vec![];

    for name in names
    {
        names_vec.push(String::from_utf8_lossy(name).to_string());
    }

    return Ok(names_vec);

}