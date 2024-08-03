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
    // Load
    #[opcode(0xA9)] LoadACImm(u8),
    #[opcode(0xAD)] LoadACAbs(u16),
    #[opcode(0xBD)] LoadACAbsX(u16),
    #[opcode(0xB9)] LoadACAbsY(u16),
    #[opcode(0xA5)] LoadACZp(u8),
    #[opcode(0xB5)] LoadACZpX(u8),
    #[opcode(0xA1)] LoadACZpXInd(u8),
    #[opcode(0xB1)] LoadACZpYInd(u8),

    #[opcode(0xA2)] LoadXImm(u8),
    #[opcode(0xAE)] LoadXAbs(u16),
    #[opcode(0xBE)] LoadXAbsY(u16),
    #[opcode(0xA6)] LoadXZp(u8),
    #[opcode(0xB6)] LoadXZpY(u8),

    #[opcode(0xA0)] LoadYImm(u8),
    #[opcode(0xAC)] LoadYAbs(u16),
    #[opcode(0xBC)] LoadYAbsX(u16),
    #[opcode(0xA4)] LoadYZp(u8),
    #[opcode(0xB4)] LoadYZpX(u8),

    //Store
    #[opcode(0x8D)] StoreACAbs(u16),
    #[opcode(0x9D)] StoreACAbsX(u16),
    #[opcode(0x99)] StoreACAbsY(u16),
    #[opcode(0x85)] StoreACZp(u8),
    #[opcode(0x95)] StoreACZpX(u8),
    #[opcode(0x81)] StoreACZpXInd(u8),
    #[opcode(0x91)] StoreACZpYInd(u8),

    #[opcode(0x8E)] StoreXAbs(u16),
    #[opcode(0x86)] StoreXZp(u8),
    #[opcode(0x96)] StoreXZpY(u8),

    #[opcode(0x8C)] StoreYAbs(u16),
    #[opcode(0x84)] StoreYZp(u8),
    #[opcode(0x94)] StoreYZpX(u8),

    // Transfert
    #[opcode(0xAA)] TransACX,
    #[opcode(0xA8)] TransACY,
    #[opcode(0xBA)] TransSPX,
    #[opcode(0x8A)] TransXAC,
    #[opcode(0x98)] TransYAC,
    #[opcode(0x9A)] TransXSP,

    // Stack
    #[opcode(0x48)] PushAC,
    #[opcode(0x08)] PushSR,
    #[opcode(0x68)] PullAC,
    #[opcode(0x28)] PullSR,

    // Shift
    #[opcode(0x0A)] ArmLShfAC,
    #[opcode(0x0E)] ArmLShfAbs(u16),
    #[opcode(0x1E)] ArmLShfAbsX(u16),
    #[opcode(0x06)] ArmLShfZp(u8),
    #[opcode(0x16)] ArmLShfZpX(u8),

    #[opcode(0x4A)] LogRShfAC,
    #[opcode(0x4E)] LogRShfAbs(u16),
    #[opcode(0x5E)] LogRShfAbsX(u16),
    #[opcode(0x46)] LogRShfZp(u8),
    #[opcode(0x56)] LogRShfZpX(u8),

    #[opcode(0x2A)] LRotAC,
    #[opcode(0x2E)] LRotAbs(u16),
    #[opcode(0x3E)] LRotAbsX(u16),
    #[opcode(0x26)] LRotZp(u8),
    #[opcode(0x36)] LRotZpX(u8),

    #[opcode(0x6A)] RRotAC,
    #[opcode(0x6E)] RRotAbs(u16),
    #[opcode(0x7E)] RRotAbsX(u16),
    #[opcode(0x66)] RRotZp(u8),
    #[opcode(0x76)] RRotZpX(u8),

    // Logic
    #[opcode(0x29)] AndImm(u8),
    #[opcode(0x2D)] AndAbs(u16),
    #[opcode(0x3D)] AndAbsX(u16),
    #[opcode(0x39)] AndAbsY(u16),
    #[opcode(0x25)] AndZp(u8),
    #[opcode(0x35)] AndZpX(u8),
    #[opcode(0x21)] AndZpXInd(u8),
    #[opcode(0x31)] AndZpYInd(u8),

    #[opcode(0x2C)] BitAbs(u16),
    #[opcode(0x24)] BitZp(u8),

    #[opcode(0x49)] EorImm(u8),
    #[opcode(0x4D)] EorAbs(u16),
    #[opcode(0x5D)] EorAbsX(u16),
    #[opcode(0x59)] EorAbsY(u16),
    #[opcode(0x45)] EorZp(u8),
    #[opcode(0x55)] EorZpX(u8),
    #[opcode(0x41)] EorZpXInd(u8),
    #[opcode(0x51)] EorZpYInd(u8),

    #[opcode(0x09)] OrImm(u8),
    #[opcode(0x0D)] OrAbs(u16),
    #[opcode(0x1D)] OrAbsX(u16),
    #[opcode(0x19)] OrAbsY(u16),
    #[opcode(0x05)] OrZp(u8),
    #[opcode(0x15)] OrZpX(u8),
    #[opcode(0x01)] OrZpXInd(u8),
    #[opcode(0x11)] OrZpYInd(u8),

    // Arithmetic
    #[opcode(0x69)] AddImm(u8),
    #[opcode(0x6D)] AddAbs(u16),
    #[opcode(0x7D)] AddAbsX(u16),
    #[opcode(0x79)] AddAbsY(u16),
    #[opcode(0x65)] AddZp(u8),
    #[opcode(0x75)] AddZpX(u8),
    #[opcode(0x61)] AddZpXInd(u8),
    #[opcode(0x71)] AddZpYInd(u8),

    #[opcode(0xC9)] CmpACImm(u8),
    #[opcode(0xCD)] CmpACAbs(u16),
    #[opcode(0xDD)] CmpACAbsX(u16),
    #[opcode(0xD9)] CmpACAbsY(u16),
    #[opcode(0xC5)] CmpACZp(u8),
    #[opcode(0xD5)] CmpACZpX(u8),
    #[opcode(0xC1)] CmpACZpXInd(u8),
    #[opcode(0xD1)] CmpACZpYInd(u8),

    #[opcode(0xE0)] CmpXImm(u8),
    #[opcode(0xEC)] CmpXAbs(u16),
    #[opcode(0xE4)] CmpXZp(u8),

    #[opcode(0xC0)] CmpYImm(u8),
    #[opcode(0xCC)] CmpYAbs(u16),
    #[opcode(0xC4)] CmpYZp(u8),

    #[opcode(0xE9)] SubImm(u8),
    #[opcode(0xED)] SubAbs(u16),
    #[opcode(0xFD)] SubAbsX(u16),
    #[opcode(0xF9)] SubAbsY(u16),
    #[opcode(0xE5)] SubZp(u8),
    #[opcode(0xF5)] SubZpX(u8),
    #[opcode(0xE1)] SubZpXInd(u8),
    #[opcode(0xF1)] SubZpYInd(u8),

    #[opcode(0xCE)] DecMemAbs(u16),
    #[opcode(0xDE)] DecMemAbsX(u16),
    #[opcode(0xC6)] DecMemZp(u8),
    #[opcode(0xD6)] DecMemZpX(u8),
    #[opcode(0xCA)] DecX,
    #[opcode(0x88)] DecY,

    #[opcode(0xEE)] IncMemAbs(u16),
    #[opcode(0xFE)] IncMemAbsX(u16),
    #[opcode(0xE6)] IncMemZp(u8),
    #[opcode(0xF6)] IncMemZpX(u8),
    #[opcode(0xE8)] IncX,
    #[opcode(0xC8)] IncY,

    // Control
    #[opcode(0x00)] Break,

    #[opcode(0x4C)] JumpAbs(u16),
    #[opcode(0x6C)] JumpAbsInc(u16),
    #[opcode(0x20)] JumpSubAbs(u16),

    #[opcode(0x40)] RetInt,
    #[opcode(0x60)] RetSub,

    // Branch
    #[opcode(0x90)] BranchNotCarry(u8),
    #[opcode(0xB0)] BranchCarry(u8),
    #[opcode(0xF0)] BranchZero(u8),
    #[opcode(0x30)] BranchNeg(u8),
    #[opcode(0xD0)] BranchNotZero(u8),
    #[opcode(0x10)] BranchNotNeg(u8),
    #[opcode(0x50)] BranchNotOver(u8),
    #[opcode(0x70)] BranchOver(u8),

    // Flags
    #[opcode(0x18)] ClrCarry,
    #[opcode(0xD8)] ClrDec,
    #[opcode(0x58)] ClrIntDis,
    #[opcode(0xB8)] ClrOver,

    #[opcode(0x38)] SetCarry,
    #[opcode(0xF8)] SetDec,
    #[opcode(0x78)] SetIntDis,
    
    // Other
    #[opcode(0xEA)] NoOp,
    #[opcode(0xFF)] EmuSignal(u8),
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
            Into::<Vec<u8>>::into(Instruction::LoadACImm(0x21)),
            vec![0xA9, 0x21]
        );
        assert_eq!(
            Into::<Vec<u8>>::into(Instruction::StoreACZp(0xA2)),
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
            Instruction::LoadACImm(0x21)
        );
        assert_eq!(
            Instruction::try_from([0x85, 0xA2, 0].as_slice()).unwrap(),
            Instruction::StoreACZp(0xA2)
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
            "LoadACImm".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadACImm 12".parse::<Instruction>().unwrap(),
            Instruction::LoadACImm(12)
        );
        assert_eq!(
            "LoadACImm 12 14"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadACImm #A2".parse::<Instruction>().unwrap(),
            Instruction::LoadACImm(0xA2)
        );
        assert_eq!(
            "LoadACImm %10011010".parse::<Instruction>().unwrap(),
            Instruction::LoadACImm(0b10011010)
        );
        assert_eq!(
            "LoadACImm 12120215125"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadACImm #12A2GH"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "LoadACImm %10032010"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );

        assert_eq!(
            "StoreACZp".parse::<Instruction>().unwrap_err().type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreACZp 12".parse::<Instruction>().unwrap(),
            Instruction::StoreACZp(12)
        );
        assert_eq!(
            "StoreACZp 12 14"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreACZp #A2".parse::<Instruction>().unwrap(),
            Instruction::StoreACZp(0xA2)
        );
        assert_eq!(
            "StoreACZp %10011010".parse::<Instruction>().unwrap(),
            Instruction::StoreACZp(0b10011010)
        );
        assert_eq!(
            "StoreACZp 12120215125"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreACZp #12A2GH"
                .parse::<Instruction>()
                .unwrap_err()
                .type_id(),
            TypeId::of::<ParsingError>()
        );
        assert_eq!(
            "StoreACZp %10032010"
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
            Instruction::LoadACImm(12)
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::LoadACImm(12)
        );
        assert_eq!(
            Instruction::StoreACZp(0b10011010)
                .to_string()
                .parse::<Instruction>()
                .unwrap(),
            Instruction::StoreACZp(0b10011010)
        );
    }
}
