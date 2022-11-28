use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

mod user;
mod stream_utils;

fn write_create(mut user: user::User, mut stream: TcpStream) -> (user::User, TcpStream)
{
    let mut bad: bool = false;

    let filename_len: u64 = stream_utils::read_long(&mut stream);

    let mut f: Vec<u8> = vec![0; filename_len as usize];

    match stream.read_exact(&mut f)
    {
        Ok(_) => {

        },
        Err(e) => {
            println!("[-] Failed to read from stream on write/create - {:?} -", e);
            bad = true;
        }
    }

    let data_len: u64 = stream_utils::read_long(&mut stream);

    let mut data: Vec<u8> = vec![0; data_len as usize];

    match stream.read_exact(&mut data)
    {
        Ok(_) => {

        },
        Err(e) => {
            println!("[-] Failed to read from stream on write/create - {:?} -", e);
            bad = true;
        }
    }

    let filename = match String::from_utf8(f)
    {
        Ok(r) => r,
        Err(e) => panic!("Failed to make u8 vector a string {:?}", e)
    };

    let mut buf: Vec<u8> = vec![];
    buf.push(0x01);

    if !user.create_or_write_file(filename, data) || bad
    {
        buf.push(0);

        if let Err(e) = stream.write_all(&buf)
        {
            println!("[-] Failed to write to stream in write/create file - {:?} -", e);
        }
    }
    else
    {
        buf.push(1);

        if let Err(e) = stream.write_all(&buf)
        {
            println!("[-] Failed to write to stream in write/create file - {:?} -", e);
        }
    }

    return (user, stream);
}

fn read(user: user::User, mut stream: TcpStream) -> (user::User, TcpStream)
{
    let mut bad: bool = false;

    let filename_len: u64 = stream_utils::read_long(&mut stream);

    let mut f: Vec<u8> = vec![0; filename_len as usize];

    match stream.read_exact(&mut f)
    {
        Ok(_) => {

        },
        Err(e) => {
            println!("[-] Failed to read from stream on write/create - {:?} -", e);
            bad = true;
        }
    }

    let filename = match String::from_utf8(f)
    {
        Ok(r) => r,
        Err(e) => panic!("Failed to make u8 vector a string {:?}", e)
    };

    let mut buf: Vec<u8> = vec![];

    let (mut data, size) = match user.get_file_data(filename)
    {
        Some((r, size)) => (r, size),
        None => {
            println!("[-] Client requested a non-existent file");
            bad = true;
            (vec![1; 1], 0)
        }
    };

    buf.push(2);

    if bad
    {
        buf.push(0);
    } else {
        buf.push(1);
        buf.append(&mut data.len().to_be_bytes().to_vec());
        buf.append(&mut data);
    }

    let text = String::from_utf8_lossy(&buf).to_string();

    if let Err(e) = stream.write_all(&buf)
    {
        println!("[-] Failed to write to stream in file read - {:?} -", e);
    }

    return (user, stream);
}

fn filenames(user: user::User, mut stream: TcpStream) -> (user::User, TcpStream)
{
    let mut buf: Vec<u8> = vec![];

    buf.push(3);

    let names = user.get_file_names();

    let mut size: usize = 0;

    size += names.len();

    for name in names.clone()
    {
        size += name.len();
    }

    buf.append(&mut size.to_be_bytes().to_vec());

    for name in names
    {
        buf.append(&mut name.as_bytes().to_vec());
        buf.push('\0' as u8);
    }

    if let Err(e) = stream.write_all(&buf)
    {
        println!("[-] Failed to write to stream in filename retrieval - {:?} -", e);
    }

    return (user, stream);
}

fn handle_client(mut stream: TcpStream) {
    let mut user = user::User::new();

    loop
    {
        let packet_id: u8 = stream_utils::read_byte(&mut stream);

        match packet_id {
            0x01_u8 => {
                println!("[+] Write call");
                (user, stream) = write_create(user, stream)
            },
            0x02_u8 => {
                println!("[+] Read call");
                (user, stream) = read(user, stream)
            },
            0x03_u8 => {
                println!("[+] Filenames retrieval");
                (user, stream) = filenames(user, stream)
            },
            _=> {
                println!("[-] Got weird request!");
            }
        }
    }
}

fn main() {
    println!("[+] Starting");

    let l = TcpListener::bind("127.0.0.1:54000");

    let listener = match l {
        Ok(r) => r,
        Err(e) => {
            println!("[-] Failed to bind service to port, exiting - {:?} -", e);
            return;
        }
    };

    for s in listener.incoming() {

        let stream = match s {
            Ok(r) => r,
            Err(e) => {
                println!("[-] Weird connection - {:?} -", e);
                continue;
            }
        };

        thread::spawn(move || {
            handle_client(stream);
        });
        
        println!("[+] Connection made!");
    }
}
