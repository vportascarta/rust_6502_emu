use std::{env, fs};

use rustemu::Vm;

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
        let _ = vm.cycle();
    }
}
