use std::{
    env, fs,
    io::{self, Read, Write},
};

use rustemu::Vm;

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
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to provide a bin file! : {} example.bin", args[0]);
        return;
    }

    fn signal_halt(vm: &mut Vm) -> Result<(), String> {
        vm.halt = true;
        Ok(())
    }

    let prog = fs::read(args[1].as_str()).unwrap();

    let mut vm = Vm::new();
    vm.define_handler(0xFF, signal_halt);
    vm.copy_memory(0, &prog);

    while !vm.halt {
        let res = vm.cycle();
        match res {
            Ok(_) => {},
            Err(err) => println!("{}", err),
        }
        pause();
    }
}
