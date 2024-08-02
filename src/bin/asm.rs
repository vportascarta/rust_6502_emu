use rustemu::isa::{Instruction, ParsingError};
use std::{
    env,
    fs::{self},
    io::Write,
};

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to provide a asm file! : {} example.asm", args[0]);
        return;
    }

    let mut binary_data = Vec::<u8>::new();

    for line in fs::read_to_string(args[1].as_str()).unwrap().lines() {
        let res = line.parse::<Instruction>();

        match res {
            Ok(ins) => binary_data.append(Into::<Vec<u8>>::into(ins).as_mut()),
            Err(ParsingError::NonBlockingError(_)) => continue,
            Err(ParsingError::BlockingError(err)) => panic!("{}", err),
        }
    }

    std::io::stdout().write_all(binary_data.as_slice()).unwrap();
}
