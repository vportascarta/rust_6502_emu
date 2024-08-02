use rustemu::isa::Instruction;
use std::{
    cmp::min, env, fs::{self}
};

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to provide a bin file! : {} example.bin", args[0]);
        return;
    }

    let prog = fs::read(args[1].as_str()).unwrap();
    let mut idx = 0;
    while idx < prog.len() {
        let raw_bytes = &prog[idx..(min(idx+3, prog.len()))];

        let instruction = Instruction::try_from(raw_bytes)
            .map_err(|err| format!("Wrong binary format : {}", err))
            .unwrap();

        println!("{}", instruction);

        idx += instruction.size()
    }
}
