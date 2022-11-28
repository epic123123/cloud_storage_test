mod user;

fn menu()
{
    println!("CREATE to write file");
    println!("READ to read file");
    println!("NAMES to get filenames");
    println!("EXIT to eat a banana");
}

fn test()
{
    println!("Test!");

    let mut user = user::User::new();

    loop{
        menu();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.contains("CRE")
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

            user.create_or_write_file(b, truedata.as_bytes().to_vec());
        }

        else if line.contains("READ")
        {
            let mut name = String::new();
            println!("Name: ");
            std::io::stdin().read_line(&mut name).unwrap();

            let a = name.replace("\r", "");
            let b = a.replace("\n", "");

            let data = match user.get_file_data(b)
            {
                Some((data, _size)) => data,
                None => {
                    println!("Something went wrong");
                    return;
                } 
            };

            let s = match String::from_utf8(data) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("{}", s.as_str());
        }

        else if line.contains("AME")
        {
            let names = user.get_file_names();
            for name in names
            {
                println!("{:?}", name);
            }
        }

        else if line.contains("XIT")
        {
            return;
        } else {
            println!("NOPE!");
        }
    }
}