use std::env;

enum Parses{
    Value,
    Pack
}

struct Config {
    pack_size: u32
}
fn main() {
    let mut args = env::args();
    let mut aux = Parses::Value;
    let mut config = Config{pack_size : 64}; 
    let mut vals = Vec::<u32>::new();

    args.next(); // removing name of executable

    for i in args { // traverse the args for parameters
        if i == "-p" {
            aux = Parses::Pack;
            continue;
        }

        match aux {
            Parses::Pack => {
                config.pack_size = i.parse::<u32>().expect("Pack value must be a positive integer");
            },
            Parses::Value => {
                vals.push(i.parse::<u32>().expect("Pack value must be a positive integer"))
            }
        }
        
    }
    for i in vals {
        let quotient = i/config.pack_size; 
        let remainder = i%config.pack_size;
        println!("packs: {} remainder: {}", quotient, remainder);
    }
}
