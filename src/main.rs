use std::{
    env, io::*, fs::*, process::exit
};

#[derive(Debug, PartialEq)]
enum Parses{
    Value,
    Pack,
    FilePath
}

#[derive(Debug, PartialEq)]
enum InputMethod{
    Args,
    Stdin,
    File(String)
}

#[derive(Debug)]
struct Config {
    pack_size: u32,
    input: InputMethod
}

fn reader(use_file: bool, br: Option<&mut BufReader<File>>, input: &mut String) -> Result<usize> {
    if use_file {
        br.expect("unreachable").read_line(input)
    } else {
        stdin().read_line(input)
    }
}

fn main() {
    let mut parse_mode = Parses::Value;
    let mut config = Config {
        pack_size : 64,
        input     : InputMethod::Args
    };
    let mut vals = Vec::<u32>::new();

    for i in env::args().skip(1) { // traverse the args for parameters
        if i == "-p" {
            parse_mode = Parses::Pack;
            continue;
        }
        else if i == "-i" {
            config.input = InputMethod::Stdin;
            continue;
        }
        else if i == "-f" {
            parse_mode = Parses::FilePath;
            continue;
        }

        match parse_mode {
            Parses::FilePath => config.input = InputMethod::File(String::from(i)),
            Parses::Pack     => config.pack_size = i.parse::<u32>().expect("Pack value must be a positive integer"),
            Parses::Value    => vals.push(i.parse::<u32>().expect("Pack value must be a positive integer")),
        }
        
    }

    if config.input != InputMethod::Args {
        
        let (use_file, mut br) = match config.input {
            InputMethod::File(path) => (true,  Some( BufReader::new(File::open(path).expect("Cold not open file")) )),
            InputMethod::Stdin      => (false, None),
            InputMethod::Args       => {exit(1)} // Unreachable due if above
        };

        loop {
            let mut input = String::new();
            
            match reader(use_file, br.as_mut(), &mut input) {
                Ok(0) => { break; }
                Ok(_) => {
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
