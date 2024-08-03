use std::{
    fs,
    io::{self, Read, Write},
};

use clap::Parser;
use rustemu::Vm;

/// Simple program to emulate a 6502 CPU
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the binary file
    #[arg(short, long)]
    prog_file_path: String,

    /// Launch in step by step mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    //write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> () {
    let args = Args::parse();

    let prog = fs::read(args.prog_file_path.as_str()).unwrap();

    let mut vm = Vm::new();
    vm.copy_memory(0, &prog);

    while !vm.halt {
        let res = vm.cycle();
        match res {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        }

        if args.debug {
            pause();
        }
    }
}
