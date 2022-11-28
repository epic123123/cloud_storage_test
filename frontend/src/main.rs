use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::{File, OpenOptions};

mod funcs;

fn menu()
{
    println!("| WRITE to write file");
    println!("| UPLOAD to upload file");
    println!("| DOWNLOAD to download file");
    println!("| READ to read file");
    println!("| NAMES to get filenames");
    println!("| EXIT to eat a banana");
}
fn main() {
    println!("- Cloud Storage Service -");

    let mut stream = match TcpStream::connect("127.0.0.1:54000")
    {
        Ok(r) => r,
        Err(e) => {
            println!("[-] Failed to connect to server - {:?} -", e);
            return;
        }
    };

    println!("[+] Connected to cloud");

    loop{
        menu();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.contains("WRITE")
        {
            let mut name = String::new();
            println!("Name: ");
            std::io::stdin().read_line(&mut name).unwrap();
            println!("Data: ");
            let mut data = String::new();
            std::io::stdin().read_line(&mut data).unwrap();

            let a = name.replace("\r", "");
            let b = a.replace("\n", "");

            let truedata = data.replace("\r", "");

            funcs::write(&mut stream, b, truedata.as_bytes().to_vec());
        }

        else if line.contains("UPLOAD")
        {
            let mut name = String::new();
            println!(" - Name: ");
            std::io::stdin().read_line(&mut name).unwrap();

            let a = name.replace("\r", "");
            let b = a.replace("\n", "");
            let c = b.clone();

            let mut file = match File::open(b)
            {
                Ok(r) => r,
                Err(e) => {
                    println!("[-] Failed to open file - {:?} -", e);
                    continue;
                }
            };

            let mut buf: Vec<u8> = vec![0; file.metadata().unwrap().len() as usize];

            if let Err(e) = file.read_to_end(&mut buf)
            {
                println!("[-] Failed to read file - {:?} -", e);
                continue;
            }

            funcs::write(&mut stream, c, buf);
        }

        else if line.contains("DOWNLOAD")
        {
            let mut name = String::new();
            println!(" - Name: ");
            std::io::stdin().read_line(&mut name).unwrap();

            let a = name.replace("\r", "");
            let b = a.replace("\n", "");
            let c = b.clone();

            let mut data = match funcs::read(&mut stream, b)
            {
                Ok(r) => r,
                Err(_) => {
                    return;
                }
            };

            let f = OpenOptions::new().write(true).create(true).open(c);

            let mut file = match f {
                Ok(r) => r,
                Err(e) => {
                    println!("[-] Failed to create file - {:?} -", e);
                    continue;
                }
            };

            if let Err(e) = file.write_all(&mut data)
            {
                println!("[-] Failed to write file on download - {:?} -", e);
                continue;
            }

            println!("[+] Download complete!");
        }

        else if line.contains("READ")
        {
            let mut name = String::new();
            println!(" - Name: ");
            std::io::stdin().read_line(&mut name).unwrap();

            let a = name.replace("\r", "");
            let b = a.replace("\n", "");
            let c = b.clone();

            let data = match funcs::read(&mut stream, b)
            {
                Ok(r) => r,
                Err(_) => {
                    return;
                }
            };
            let data_s: String = String::from_utf8_lossy(&data).to_string();

            println!("Data from {}", c.as_str());
            println!("{}", data_s.as_str());
        }

        else if line.contains("NAMES")
        {
            let names: Vec<String> = match funcs::filenames(&mut stream)
            {
                Ok(r) => r,
                Err(_) => {
                    return;
                }
            };
            for name in names
            {
                println!("{:?}", name);
            }
        }

        else if line.contains("EXIT")
        {
            return;
        } else {
            println!("NOPE!");
        }
    }
}
