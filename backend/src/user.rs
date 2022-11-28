use std::vec;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use rand::{distributions::Alphanumeric, Rng};

pub struct User {
    files: HashMap<String, String>,
}

impl User {

    pub fn new() -> User
    {
        User { files: HashMap::new() }
    }

    pub fn create_or_write_file(&mut self, name: String, data: Vec<u8>) -> bool
    {
        let name2 = name.clone();

        let r = self.files.get(&name);

        let mut exists = true;

        let temp = String::from("");

        let real_name = match r {
            Some(r) => r,
            None => {
                exists = false;
                &temp
            }
        };

        let mut file;

        if exists 
        {
            let f = OpenOptions::new().write(true).read(true).open(real_name);

            file = match f {
                Ok(r) => r,
                Err(e) => {
                    println!("[-] Failed to open file - {:?} -", e);
                    return false;
                }
            };
        }
        else
        {
            let real_name: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            
            let f = OpenOptions::new().write(true).read(true).create(true).open(&real_name);

            file = match f {
                Ok(r) => r,
                Err(e) => {
                    println!("[-] Failed to open file - {:?} -", e);
                    return false;
                }
            };

            self.files.insert(name2, real_name);
        }


        if let Err(e) = file.write_all(&data) {
            println!("[-] Failed to write to file - {:?} -", e);
            return false;
        }

        println!("[+] Wrote file!");

        return true;
    }

    pub fn get_file_names(&self) -> Vec<String> 
    {
        let mut files: Vec<String> = vec![];

        for (name, _) in &self.files {
            files.push(name.to_string());
        }

        return files;
    }

    pub fn get_file_data(&self, name: String) -> Option<(Vec<u8>, usize)>
    {
        let r = self.files.get(&name);

        let real_name = match r {
            Some(r) => r,
            None => {
                return None;
            }
        };

        let f = File::open(real_name);

        let mut file = match f {
            Ok(r) => r,
            Err(e) => {
                println!("[-] Failed to open file - {:?} -", e);
                return None;
            }
        };
        // r.metadata().unwrap().len().try_into().unwrap()
        let mut buf: Vec<u8> = vec![0; file.metadata().unwrap().len() as usize];
        let len = match file.read_to_end(&mut buf)
        {
            Ok(r) => r,
            Err(e) => {
                println!("[-] Failed to read file - {:?} -", e);
                return None;
            }
        };
        return Some((buf, len));
    }
}