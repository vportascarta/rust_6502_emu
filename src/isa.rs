use std::{
    fmt::{self},
    str::FromStr,
};

use rustemu_macros::EmuInstruction;

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Register {
    AC = 0x00,
    X = 0x01,
    Y = 0x02,
    PCL = 0x03,
    PCH = 0x04,
    SP = 0x05,
    SR = 0x06,
    IRQ = 0x07,
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterFlag {
    Carry = 0x00,
    Zero = 0x01,
    Interrupt = 0x02,
    Decimal = 0x03,
    Break = 0x04,
    Overflow = 0x06,
    Negative = 0x07,
}

#[derive(EmuInstruction, PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    #[opcode(0x00)]
    Break,
    #[opcode(0x4C)]
    JumpAbs(u16),
    #[opcode(0x69)]
    AddCImm(u8),
    #[opcode(0x85)]
    StoreAccZp(u8),
    #[opcode(0x90)]
    BranchCC(u8),
    #[opcode(0xB0)]
    BranchCS(u8),
    #[opcode(0xF0)]
    BranchZ(u8),
    #[opcode(0xA9)]
    LoadAccImm(u8),
    #[opcode(0xEA)]
    NoOp,
    #[opcode(0xFF)]
    EmuSignal(u8),
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_instruction_to_binary() -> () {
        assert_eq!(Into::<Vec<u8>>::into(Instruction::NoOp), vec![0xEA]);
        assert_eq!(Into::<Vec<u8>>::into(Instruction::Break), vec![0x00]);
        assert_eq!(
            Into::<Vec<u8>>::into(Instruction::JumpAbs(0xDA21)),
            vec![0x4C, 0x21, 0xDA]
        );
        assert_eq!(
            Into::<Vec<u8>>::into(Instruction::LoadAccImm(0x21)),
            vec![0xA9, 0x21]
        );
        assert_eq!(
            Into::<Vec<u8>>::into(Instruction::StoreAccZp(0xA2)),
            vec![0x85, 0xA2]
        );
    }

    #[test]
    fn test_instruction_from_binary() -> () {
        assert_eq!(
            Instruction::try_from([0xEA, 0, 0].as_slice()).unwrap(),
            Instruction::NoOp
        );
        assert_eq!(
            Instruction::try_from([0x00, 0, 0].as_slice()).unwrap(),
            Instruction::Break
        );
        assert_eq!(
            Instruction::try_from([0x4C, 0x21, 0xDA].as_slice()).unwrap(),
            Instruction::JumpAbs(0xDA21)
        );
        assert_eq!(
            Instruction::try_from([0xA9, 0x21, 0].as_slice()).unwrap(),
            Instruction::LoadAccImm(0x21)
        );
        assert_eq!(
            Instruction::try_from([0x85, 0xA2, 0].as_slice()).unwrap(),
            Instruction::StoreAccZp(0xA2)
        );
    }

    #[test]
    fn test_instruction_from_string() -> () {
        assert_eq!(
            "".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "; test".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!("NoOp".parse::<Instruction>().unwrap(), Instruction::NoOp);
        assert_eq!(
            "NoOpop".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "NoOp 1212".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!("Break".parse::<Instruction>().unwrap(), Instruction::Break);
        assert_eq!(
            "Breakop".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "Break 1212".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!(
            "JumpAbs".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "JumpAbs 1212".parse::<Instruction>().unwrap(),
            Instruction::JumpAbs(1212)
        );
        assert_eq!(
            "JumpAbs 1212 1414"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "JumpAbs #12A2".parse::<Instruction>().unwrap(),
            Instruction::JumpAbs(0x12A2)
        );
        assert_eq!(
            "JumpAbs %1001101010011010".parse::<Instruction>().unwrap(),
            Instruction::JumpAbs(0b1001101010011010)
        );
        assert_eq!(
            "JumpAbs 12120215125"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "JumpAbs #12A2GH"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "JumpAbs %10032010"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!(
            "LoadAccImm".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadAccImm 12".parse::<Instruction>().unwrap(),
            Instruction::LoadAccImm(12)
        );
        assert_eq!(
            "LoadAccImm 12 14"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadAccImm #A2".parse::<Instruction>().unwrap(),
            Instruction::LoadAccImm(0xA2)
        );
        assert_eq!(
            "LoadAccImm %10011010".parse::<Instruction>().unwrap(),
            Instruction::LoadAccImm(0b10011010)
        );
        assert_eq!(
            "LoadAccImm 12120215125"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadAccImm #12A2GH"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadAccImm %10032010"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!(
            "StoreAccZp".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreAccZp 12".parse::<Instruction>().unwrap(),
            Instruction::StoreAccZp(12)
        );
        assert_eq!(
            "StoreAccZp 12 14"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreAccZp #A2".parse::<Instruction>().unwrap(),
            Instruction::StoreAccZp(0xA2)
        );
        assert_eq!(
            "StoreAccZp %10011010".parse::<Instruction>().unwrap(),
            Instruction::StoreAccZp(0b10011010)
        );
        assert_eq!(
            "StoreAccZp 12120215125"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreAccZp #12A2GH"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreAccZp %10032010"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
    }

    #[test]
    fn test_instruction_to_from_string() -> () {
        assert_eq!(
            Instruction::NoOp
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::NoOp
        );
        assert_eq!(
            Instruction::Break
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::Break
        );
        assert_eq!(
            Instruction::JumpAbs(0x12A2)
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::JumpAbs(0x12A2)
        );
        assert_eq!(
            Instruction::LoadAccImm(12)
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::LoadAccImm(12)
        );
        assert_eq!(
            Instruction::StoreAccZp(0b10011010)
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::StoreAccZp(0b10011010)
        );
    }
}
