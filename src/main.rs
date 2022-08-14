use std::env;
use std::io;

#[derive(Debug, PartialEq)]
enum Parses{
    Value,
    Pack
}

#[derive(Debug)]
struct Config {
    pack_size: u32,
    from_stdin: bool
}

fn main() {
    let mut aux = Parses::Value;
    let mut config = Config{pack_size : 64, from_stdin : false}; 
    let mut vals = Vec::<u32>::new();

    for i in env::args().skip(1) { // traverse the args for parameters
        if i == "-p" {
            aux = Parses::Pack;
            continue;
        }
        else if i == "-i" {
            config.from_stdin = true;
            continue;
        }

        match aux {
            Parses::Pack => config.pack_size = i.parse::<u32>().expect("Pack value must be a positive integer"),
            Parses::Value => vals.push(i.parse::<u32>().expect("Pack value must be a positive integer"))
        }
        
    }
    if config.from_stdin {
        loop {
            let mut input = String::new();

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
