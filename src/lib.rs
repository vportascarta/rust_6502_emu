use std::{cmp::min, collections::HashMap, fmt};

use isa::{Instruction, Register, RegisterFlag};

pub mod isa;

type SignalFunction = fn(&mut Vm) -> Result<(), String>;

pub struct Vm {
    registers: [u8; 8],
    memory: [u8; 64 * 1024],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: [0; 64 * 1024],
            signal_handlers: HashMap::new(),
            halt: false,
        }
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    pub fn get_flag(&self, register_flag: RegisterFlag) -> bool {
        self.registers[Register::SR as usize] & (1 << register_flag as u8) > 0
    }

    pub fn set_flag(&mut self, register_flag: RegisterFlag, value: bool) {
        //println!("FLAG write {:?}, {}", register_flag, value);
        if value {
            self.registers[Register::SR as usize] |=
                0b0000_0001_u8.rotate_left(register_flag as u32);
        } else {
            self.registers[Register::SR as usize] &=
                0b1111_1110_u8.rotate_left(register_flag as u32);
        }
    }

    pub fn get_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    pub fn set_register(&mut self, register: Register, value: u8) -> () {
        //println!("REG write {:?}, {}", register, value);
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
        let mut pc: usize = (self.get_register(Register::PCH) as usize) << 8
            | (self.get_register(Register::PCL) as usize);

        let raw_bytes = &self.memory[pc..(min(pc + 3, self.memory.len()))];

        let instruction = Instruction::try_from(raw_bytes)
            .map_err(|err| format!("Wrong binary format : {}", err))?;

        pc += instruction.size();
        println!("{}", instruction);

        match instruction {
            Instruction::LoadACImm(op) => {
                self.set_register(Register::AC, op);
                self.update_flag_zero(op.into());
                self.update_flag_negative(op.into());
            }
            Instruction::LoadACAbs(_) => todo!(),
            Instruction::LoadACAbsX(_) => todo!(),
            Instruction::LoadACAbsY(_) => todo!(),
            Instruction::LoadACZp(_) => todo!(),
            Instruction::LoadACZpX(_) => todo!(),
            Instruction::LoadACZpXInd(_) => todo!(),
            Instruction::LoadACZpYInd(_) => todo!(),
            Instruction::LoadXImm(_) => todo!(),
            Instruction::LoadXAbs(_) => todo!(),
            Instruction::LoadXAbsY(_) => todo!(),
            Instruction::LoadXZp(_) => todo!(),
            Instruction::LoadXZpY(_) => todo!(),
            Instruction::LoadYImm(_) => todo!(),
            Instruction::LoadYAbs(_) => todo!(),
            Instruction::LoadYAbsX(_) => todo!(),
            Instruction::LoadYZp(_) => todo!(),
            Instruction::LoadYZpX(_) => todo!(),
            Instruction::StoreACAbs(op) => {
                let value = self.get_register(Register::AC);
                self.write_memory(op, value)?
            }
            Instruction::StoreACAbsX(_) => todo!(),
            Instruction::StoreACAbsY(_) => todo!(),
            Instruction::StoreACZp(op) => {
                let value = self.get_register(Register::AC);
                self.write_memory(op.into(), value)?
            }
            Instruction::StoreACZpX(_) => todo!(),
            Instruction::StoreACZpXInd(_) => todo!(),
            Instruction::StoreACZpYInd(_) => todo!(),
            Instruction::StoreXAbs(_) => todo!(),
            Instruction::StoreXZp(_) => todo!(),
            Instruction::StoreXZpY(_) => todo!(),
            Instruction::StoreYAbs(_) => todo!(),
            Instruction::StoreYZp(_) => todo!(),
            Instruction::StoreYZpX(_) => todo!(),
            Instruction::TransACX => todo!(),
            Instruction::TransACY => todo!(),
            Instruction::TransSPX => todo!(),
            Instruction::TransXAC => todo!(),
            Instruction::TransYAC => todo!(),
            Instruction::TransXSP => todo!(),
            Instruction::PushAC => todo!(),
            Instruction::PushSR => todo!(),
            Instruction::PullAC => todo!(),
            Instruction::PullSR => todo!(),
            Instruction::ArmLShfAC => {
                let value = self.get_register(Register::AC);
                let res = (value as u16) << 1;
                self.update_flag_carry(res);
                self.set_register(Register::AC, (res & 0xFF) as u8);
            }
            Instruction::ArmLShfAbs(_) => todo!(),
            Instruction::ArmLShfAbsX(_) => todo!(),
            Instruction::ArmLShfZp(_) => todo!(),
            Instruction::ArmLShfZpX(_) => todo!(),
            Instruction::LogRShfAC => todo!(),
            Instruction::LogRShfAbs(_) => todo!(),
            Instruction::LogRShfAbsX(_) => todo!(),
            Instruction::LogRShfZp(_) => todo!(),
            Instruction::LogRShfZpX(_) => todo!(),
            Instruction::LRotAC => todo!(),
            Instruction::LRotAbs(_) => todo!(),
            Instruction::LRotAbsX(_) => todo!(),
            Instruction::LRotZp(_) => todo!(),
            Instruction::LRotZpX(_) => todo!(),
            Instruction::RRotAC => todo!(),
            Instruction::RRotAbs(_) => todo!(),
            Instruction::RRotAbsX(_) => todo!(),
            Instruction::RRotZp(_) => todo!(),
            Instruction::RRotZpX(_) => todo!(),
            Instruction::AndImm(_) => todo!(),
            Instruction::AndAbs(_) => todo!(),
            Instruction::AndAbsX(_) => todo!(),
            Instruction::AndAbsY(_) => todo!(),
            Instruction::AndZp(_) => todo!(),
            Instruction::AndZpX(_) => todo!(),
            Instruction::AndZpXInd(_) => todo!(),
            Instruction::AndZpYInd(_) => todo!(),
            Instruction::BitAbs(_) => todo!(),
            Instruction::BitZp(_) => todo!(),
            Instruction::EorImm(_) => todo!(),
            Instruction::EorAbs(_) => todo!(),
            Instruction::EorAbsX(_) => todo!(),
            Instruction::EorAbsY(_) => todo!(),
            Instruction::EorZp(_) => todo!(),
            Instruction::EorZpX(_) => todo!(),
            Instruction::EorZpXInd(_) => todo!(),
            Instruction::EorZpYInd(_) => todo!(),
            Instruction::OrImm(_) => todo!(),
            Instruction::OrAbs(_) => todo!(),
            Instruction::OrAbsX(_) => todo!(),
            Instruction::OrAbsY(_) => todo!(),
            Instruction::OrZp(_) => todo!(),
            Instruction::OrZpX(_) => todo!(),
            Instruction::OrZpXInd(_) => todo!(),
            Instruction::OrZpYInd(_) => todo!(),
            Instruction::AddImm(op) => {
                let carry = self.get_flag(RegisterFlag::Carry);
                let acc = self.get_register(Register::AC);
                let res = (acc as u16) + (op as u16) + (carry as u16);

                // Set flags
                self.update_flag_carry(res);
                self.update_flag_zero(res);
                self.update_flag_overflow(acc as u16, res);
                self.update_flag_negative(res);

                self.set_register(Register::AC, (res & 0xFF) as u8)
            },
            Instruction::AddAbs(op) => {
                let carry = self.get_flag(RegisterFlag::Carry);
                let acc = self.get_register(Register::AC);
                let mem = self.read_memory(op).unwrap();
                let res = (acc as u16) + (mem as u16) + (carry as u16);

                // Set flags
                self.update_flag_carry(res);
                self.update_flag_zero(res);
                self.update_flag_overflow(acc as u16, res);
                self.update_flag_negative(res);

                self.set_register(Register::AC, (res & 0xFF) as u8)
            },
            Instruction::AddAbsX(_) => todo!(),
            Instruction::AddAbsY(_) => todo!(),
            Instruction::AddZp(_) => todo!(),
            Instruction::AddZpX(_) => todo!(),
            Instruction::AddZpXInd(_) => todo!(),
            Instruction::AddZpYInd(_) => todo!(),
            Instruction::CmpACImm(_) => todo!(),
            Instruction::CmpACAbs(_) => todo!(),
            Instruction::CmpACAbsX(_) => todo!(),
            Instruction::CmpACAbsY(_) => todo!(),
            Instruction::CmpACZp(_) => todo!(),
            Instruction::CmpACZpX(_) => todo!(),
            Instruction::CmpACZpXInd(_) => todo!(),
            Instruction::CmpACZpYInd(_) => todo!(),
            Instruction::CmpXImm(_) => todo!(),
            Instruction::CmpXAbs(_) => todo!(),
            Instruction::CmpXZp(_) => todo!(),
            Instruction::CmpYImm(_) => todo!(),
            Instruction::CmpYAbs(_) => todo!(),
            Instruction::CmpYZp(_) => todo!(),
            Instruction::SubImm(_) => todo!(),
            Instruction::SubAbs(_) => todo!(),
            Instruction::SubAbsX(_) => todo!(),
            Instruction::SubAbsY(_) => todo!(),
            Instruction::SubZp(_) => todo!(),
            Instruction::SubZpX(_) => todo!(),
            Instruction::SubZpXInd(_) => todo!(),
            Instruction::SubZpYInd(_) => todo!(),
            Instruction::DecMemAbs(_) => todo!(),
            Instruction::DecMemAbsX(_) => todo!(),
            Instruction::DecMemZp(_) => todo!(),
            Instruction::DecMemZpX(_) => todo!(),
            Instruction::DecX => todo!(),
            Instruction::DecY => todo!(),
            Instruction::IncMemAbs(_) => todo!(),
            Instruction::IncMemAbsX(_) => todo!(),
            Instruction::IncMemZp(_) => todo!(),
            Instruction::IncMemZpX(_) => todo!(),
            Instruction::IncX => todo!(),
            Instruction::IncY => todo!(),
            Instruction::Break => todo!(),
            Instruction::JumpAbs(op) => pc = op.into(),
            Instruction::JumpAbsInc(_) => todo!(),
            Instruction::JumpSubAbs(_) => todo!(),
            Instruction::RetInt => todo!(),
            Instruction::RetSub => todo!(),
            Instruction::BranchNotCarry(op) => {
                if !self.get_flag(RegisterFlag::Carry) {
                    pc = self.decode_relative(op)
                }
            }
            Instruction::BranchCarry(op) => {
                if self.get_flag(RegisterFlag::Carry) {
                    pc = self.decode_relative(op)
                }
            }
            Instruction::BranchZero(op) => {
                if self.get_flag(RegisterFlag::Zero) {
                    pc = self.decode_relative(op)
                }
            }
            Instruction::BranchNeg(_) => todo!(),
            Instruction::BranchNotZero(_) => todo!(),
            Instruction::BranchNotNeg(_) => todo!(),
            Instruction::BranchNotOver(_) => todo!(),
            Instruction::BranchOver(_) => todo!(),
            Instruction::ClrCarry => self.set_flag(RegisterFlag::Carry, false),
            Instruction::ClrDec => todo!(),
            Instruction::ClrIntDis => todo!(),
            Instruction::ClrOver => todo!(),
            Instruction::SetCarry => todo!(),
            Instruction::SetDec => todo!(),
            Instruction::SetIntDis => todo!(),
            Instruction::NoOp => {}
            Instruction::Jam => self.halt = true,
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
        println!("{}", self);

        Ok(())
    }

    // Addressing mode decoding
    fn decode_absolute_x(&self, value: u16) -> u16 {
        let x = self.get_register(Register::X);
        value + x as u16
    }

    fn decode_absolute_y(&self, value: u16) -> u16 {
        let y = self.get_register(Register::Y);
        value + y as u16
    }

    fn decode_absolute_indirect(&self, value: u16) -> u16 {
        let mem_low = self.read_memory(value).unwrap();
        let mem_high = self.read_memory(value + 1).unwrap();
        ((mem_high as u16) << 8) | (mem_low as u16)
    }

    fn decode_relative(&self, value: u8) -> usize {
        let pc: usize = (self.get_register(Register::PCH) as usize) << 8
            | (self.get_register(Register::PCL) as usize);

        (pc as isize + isize::from((value) as i8)) as usize
    }

    fn decode_zeropage_x(&self, value: u8) -> u8 {
        let x = self.get_register(Register::X);
        value.wrapping_add(x)
    }

    fn decode_zeropage_y(&self, value: u8) -> u8 {
        let y = self.get_register(Register::Y);
        value.wrapping_add(y)
    }

    fn decode_zeropage_x_indirect(&self, value: u8) -> u16 {
        let x = self.get_register(Register::X);
        let addr = value.wrapping_add(x) as u16;

        let mem_low = self.read_memory(addr).unwrap();
        let mem_high = self.read_memory(addr + 1).unwrap();
        ((mem_high as u16) << 8) | (mem_low as u16)
    }

    fn decode_zeropage_indirect_y(&self, value: u8) -> u16 {
        let addr = value as u16;

        let mem_low = self.read_memory(addr).unwrap();
        let mem_high = self.read_memory(addr + 1).unwrap();

        let y = self.get_register(Register::X);

        (((mem_high as u16) << 8) | (mem_low as u16)) + y as u16
    }

    // Set flags - Use u16 to simplify flag checks
    fn update_flag_carry(&mut self, value: u16) {
        if value > 255 {
            self.set_flag(RegisterFlag::Carry, true)
        } else {
            self.set_flag(RegisterFlag::Carry, false)
        }
    }

    fn update_flag_zero(&mut self, value: u16) {
        if value & 0xFF == 0 {
            self.set_flag(RegisterFlag::Zero, true)
        } else {
            self.set_flag(RegisterFlag::Zero, false)
        }
    }

    fn update_flag_overflow(&mut self, old: u16, value: u16) {
        if value & 0x80 != old & 0x80 {
            self.set_flag(RegisterFlag::Overflow, true)
        } else {
            self.set_flag(RegisterFlag::Overflow, false)
        }
    }

    fn update_flag_negative(&mut self, value: u16) {
        if value & 0x80 > 0 {
            self.set_flag(RegisterFlag::Negative, true)
        } else {
            self.set_flag(RegisterFlag::Negative, false)
        }
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
