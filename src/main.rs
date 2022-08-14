use std::env;
use std::io;
use std::fs::*;

#[derive(Debug, PartialEq)]
enum Parses{
    Value,
    Pack,
    FromFile
}

#[derive(Debug)]
struct Config {
    pack_size: u32,
    use_file: bool,
    from_file: Option<String>
}

fn main() {
    let mut aux = Parses::Value;
    let mut config = Config{pack_size : 64,
                            use_file : false,
                            from_file : None}; 
    let mut vals = Vec::<u32>::new();

    for i in env::args().skip(1) { // traverse the args for parameters
        if i == "-p" {
            aux = Parses::Pack;
            continue;
        }
        else if i == "-i" {
            config.use_file = true;
            continue;
        }
        else if i == "-f" {
            config.use_file = true;
            aux = Parses::FromFile;
            continue;
        }

        match aux {
            Parses::Pack => config.pack_size = i.parse::<u32>().expect("Pack value must be a positive integer"),
            Parses::Value => vals.push(i.parse::<u32>().expect("Pack value must be a positive integer")),
            Parses::FromFile => config.from_file = Some(i)
        }
        
    }
    if config.use_file {
        
        loop {
            let mut input = String::new();
            let file = match config.from_file {
                Some(_) => String::new(),  //File::open(path).expect("Could not open file").
                None => String::new()// 
            };
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if !input.is_empty() {
                        vals.push( input.trim().parse::<u32>().expect("Pack Value must be positive integers"));
                    }
                    else {
                        break;
                    }
                },
                Err(_) =>  {}
            }
        } 
    }
    for i in vals {
        let quotient = i/config.pack_size; 
        let remainder = i%config.pack_size;
        println!("packs: {} remainder: {}", quotient, remainder);
    }
}
