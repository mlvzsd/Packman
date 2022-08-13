use std::env;

#[derive(Debug)]
enum Parses{
    Value,
    Pack
}

#[derive(Debug)]
struct Config {
    pack_size: u32
}

fn main() {
    let mut aux = Parses::Value;
    let mut config = Config{pack_size : 64}; 
    let mut vals = Vec::<u32>::new();

    for i in env::args().skip(1) { // traverse the args for parameters
        if i == "-p" {
            aux = Parses::Pack;
            continue;
        }

        match aux {
            Parses::Pack => {
                config.pack_size = i.parse::<u32>().expect("Pack value must be a positive integer");
            },
            Parses::Value => vals.push(i.parse::<u32>().expect("Pack value must be a positive integer"))
        }
        
    }
    for i in vals {
        let quotient = i/config.pack_size; 
        let remainder = i%config.pack_size;
        println!("packs: {} remainder: {}", quotient, remainder);
    }
}
