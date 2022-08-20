use std::{
    env, io::*, fs::*
};

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

fn reader(use_file: bool, br: Option<&mut BufReader<File>>, input: &mut String) -> Result<usize> {
    if use_file { br.expect("unreachable").read_line(input) }
    else { stdin().read_line(input) }
}

fn main() {
    let mut aux = Parses::Value;
    let mut config = Config{pack_size : 64,
                            use_file  : false,
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
            Parses::FromFile => config.from_file = Some(String::from(i))
        }
        
    }

    // let cfg = config;

    if config.use_file {
        
        let (use_file, mut br) = match config.from_file {
            Some(p) => (true,  Some(BufReader::new(File::open(p).expect("Cold not open file")))),
            None    => (false, None)
        };

        loop {
            let mut input = String::new();
            
            match reader(use_file, br.as_mut(), &mut input) {
                Ok(0) => { break; }
                Ok(n) => {
                    if !input.is_empty() {
                        vals.push(input.trim().parse::<u32>().expect("Pack Value must be positive integers"));
                    }
                },
                Err(_) => { }
            }
        } 
    }
    for i in vals {
        let quotient  = i/config.pack_size; 
        let remainder = i%config.pack_size;
        println!("packs: {} remainder: {}", quotient, remainder);
    }
}
