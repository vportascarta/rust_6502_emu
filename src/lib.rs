use std::{cmp::min, collections::HashMap, fmt};

use isa::{Instruction, Register, RegisterFlag};

pub mod isa;

type SignalFunction = fn(&mut Vm) -> Result<(), String>;

pub struct Vm {
    registers: [u8; 8],
    memory: [u8; 64 * 1024],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: [0; 64 * 1024],
            signal_handlers: HashMap::new(),
            halt: false
        }
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    pub fn get_flag(&self, register_flag: RegisterFlag) -> bool {
        self.registers[Register::SR as usize] & (1 << register_flag as u8) > 0
    }

    pub fn set_flag(&mut self, register_flag: RegisterFlag, value: bool) {
        if value {
            self.registers[Register::SR as usize] |=
                0b0000_0001_u8.rotate_right(register_flag as u32);
        } else {
            self.registers[Register::SR as usize] &=
                0b1111_1110_u8.rotate_left(register_flag as u32);
        }
    }

    pub fn get_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    pub fn set_register(&mut self, register: Register, value: u8) -> () {
        self.registers[register as usize] = value
    }

    pub fn read_memory(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.memory.len() {
            Some(self.memory[addr as usize])
        } else {
            None
        }
    }

    pub fn write_memory(&mut self, addr: u16, value: u8) -> Result<(), String> {
        println!("MEM write 0x{:04x}, 0x{:02x}", addr, value);
        if (addr as usize) < self.memory.len() {
            self.memory[addr as usize] = value;
            Ok(())
        } else {
            Err(format!("Wrong memory write address 0x{:02x}", addr))
        }
    }

    pub fn copy_memory(&mut self, from_addr: usize, value: &[u8]) -> () {
        for (idx, addr) in (from_addr..from_addr + value.len()).enumerate() {
            self.memory[addr] = value[idx]
        }
    }

    pub fn cycle(&mut self) -> Result<(), String> {
        println!("{}", self);
        let mut pc: usize = (self.get_register(Register::PCH) as usize) << 8
            | (self.get_register(Register::PCL) as usize);

        let raw_bytes = &self.memory[pc..(min(pc + 3, self.memory.len()))];

        let instruction = Instruction::try_from(raw_bytes)
            .map_err(|err| format!("Wrong binary format : {}", err))?;

        pc += instruction.size();
        println!("{}", instruction);

        match instruction {
            Instruction::NoOp => {}
            Instruction::Break => {}
            Instruction::LoadAccImm(op) => self.set_register(Register::AC, op),
            Instruction::StoreAccZp(op) => {
                let value = self.get_register(Register::AC);
                self.write_memory(op as u16, value)?
            }
            Instruction::JumpAbs(op) => pc = op as usize,
            Instruction::EmuSignal(op) => {
                let fn_signal = self
                    .signal_handlers
                    .get(&op)
                    .ok_or(format!("Unknown signal : {}", op))?;

                fn_signal(self)?
            }
        }

        self.set_register(Register::PCH, ((pc & 0xFF00) >> 8) as u8);
        self.set_register(Register::PCL, (pc & 0xFF) as u8);

        Ok(())
    }
}

impl fmt::Display for Vm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Registers : AC 0x{:02x} | X 0x{:02x} | Y 0x{:02x} | PCL 0x{:02x} | PCH 0x{:02x} | SP 0x{:02x} | SR 0x{:02x} | IRQ 0x{:02x}",
            self.get_register(Register::AC),
            self.get_register(Register::X),
            self.get_register(Register::Y),
            self.get_register(Register::PCL),
            self.get_register(Register::PCH),
            self.get_register(Register::SP),
            self.get_register(Register::SR),
            self.get_register(Register::IRQ)
        )
    }
}
