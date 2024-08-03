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
    #[opcode(0xA9)] #[asmstr("LDA")] #[addrmode("imm")] LoadACImm(u8),
    #[opcode(0xAD)] #[asmstr("LDA")] #[addrmode("abs")] LoadACAbs(u16),
    #[opcode(0xBD)] #[asmstr("LDA")] #[addrmode("abx")] LoadACAbsX(u16),
    #[opcode(0xB9)] #[asmstr("LDA")] #[addrmode("aby")] LoadACAbsY(u16),
    #[opcode(0xA5)] #[asmstr("LDA")] #[addrmode("zpm")] LoadACZp(u8),
    #[opcode(0xB5)] #[asmstr("LDA")] #[addrmode("zpx")] LoadACZpX(u8),
    #[opcode(0xA1)] #[asmstr("LDA")] #[addrmode("zxi")] LoadACZpXInd(u8),
    #[opcode(0xB1)] #[asmstr("LDA")] #[addrmode("zyi")] LoadACZpYInd(u8),

    #[opcode(0xA2)] #[asmstr("LDX")] #[addrmode("imm")] LoadXImm(u8),
    #[opcode(0xAE)] #[asmstr("LDX")] #[addrmode("abs")] LoadXAbs(u16),
    #[opcode(0xBE)] #[asmstr("LDX")] #[addrmode("aby")] LoadXAbsY(u16),
    #[opcode(0xA6)] #[asmstr("LDX")] #[addrmode("zpm")] LoadXZp(u8),
    #[opcode(0xB6)] #[asmstr("LDX")] #[addrmode("zpy")] LoadXZpY(u8),

    #[opcode(0xA0)] #[asmstr("LDY")] #[addrmode("imm")] LoadYImm(u8),
    #[opcode(0xAC)] #[asmstr("LDY")] #[addrmode("abs")] LoadYAbs(u16),
    #[opcode(0xBC)] #[asmstr("LDY")] #[addrmode("abx")] LoadYAbsX(u16),
    #[opcode(0xA4)] #[asmstr("LDY")] #[addrmode("zpm")] LoadYZp(u8),
    #[opcode(0xB4)] #[asmstr("LDY")] #[addrmode("zpx")] LoadYZpX(u8),

    //Store
    #[opcode(0x8D)] #[asmstr("STA")] #[addrmode("abs")] StoreACAbs(u16),
    #[opcode(0x9D)] #[asmstr("STA")] #[addrmode("abx")] StoreACAbsX(u16),
    #[opcode(0x99)] #[asmstr("STA")] #[addrmode("aby")] StoreACAbsY(u16),
    #[opcode(0x85)] #[asmstr("STA")] #[addrmode("zpm")] StoreACZp(u8),
    #[opcode(0x95)] #[asmstr("STA")] #[addrmode("zpx")] StoreACZpX(u8),
    #[opcode(0x81)] #[asmstr("STA")] #[addrmode("zxi")] StoreACZpXInd(u8),
    #[opcode(0x91)] #[asmstr("STA")] #[addrmode("zyi")] StoreACZpYInd(u8),

    #[opcode(0x8E)] #[asmstr("STX")] #[addrmode("abs")] StoreXAbs(u16),
    #[opcode(0x86)] #[asmstr("STX")] #[addrmode("zpm")] StoreXZp(u8),
    #[opcode(0x96)] #[asmstr("STX")] #[addrmode("zpy")] StoreXZpY(u8),

    #[opcode(0x8C)] #[asmstr("STY")] #[addrmode("abs")] StoreYAbs(u16),
    #[opcode(0x84)] #[asmstr("STY")] #[addrmode("zpm")] StoreYZp(u8),
    #[opcode(0x94)] #[asmstr("STY")] #[addrmode("zpx")] StoreYZpX(u8),

    // Transfert
    #[opcode(0xAA)] #[asmstr("TAX")] #[addrmode("imp")] TransACX,
    #[opcode(0xA8)] #[asmstr("TAY")] #[addrmode("imp")] TransACY,
    #[opcode(0xBA)] #[asmstr("TSX")] #[addrmode("imp")] TransSPX,
    #[opcode(0x8A)] #[asmstr("TXA")] #[addrmode("imp")] TransXAC,
    #[opcode(0x98)] #[asmstr("TYA")] #[addrmode("imp")] TransYAC,
    #[opcode(0x9A)] #[asmstr("TXS")] #[addrmode("imp")] TransXSP,

    // Stack
    #[opcode(0x48)] #[asmstr("PHA")] #[addrmode("imp")] PushAC,
    #[opcode(0x08)] #[asmstr("PHP")] #[addrmode("imp")] PushSR,
    #[opcode(0x68)] #[asmstr("PLA")] #[addrmode("imp")] PullAC,
    #[opcode(0x28)] #[asmstr("PLP")] #[addrmode("imp")] PullSR,

    // Shift
    #[opcode(0x0A)] #[asmstr("ASL")] #[addrmode("imp")] ArmLShfAC,
    #[opcode(0x0E)] #[asmstr("ASL")] #[addrmode("abs")] ArmLShfAbs(u16),
    #[opcode(0x1E)] #[asmstr("ASL")] #[addrmode("abx")] ArmLShfAbsX(u16),
    #[opcode(0x06)] #[asmstr("ASL")] #[addrmode("zpm")] ArmLShfZp(u8),
    #[opcode(0x16)] #[asmstr("ASL")] #[addrmode("zpx")] ArmLShfZpX(u8),

    #[opcode(0x4A)] #[asmstr("LSR")] #[addrmode("imp")] LogRShfAC,
    #[opcode(0x4E)] #[asmstr("LSR")] #[addrmode("abs")] LogRShfAbs(u16),
    #[opcode(0x5E)] #[asmstr("LSR")] #[addrmode("abx")] LogRShfAbsX(u16),
    #[opcode(0x46)] #[asmstr("LSR")] #[addrmode("zpm")] LogRShfZp(u8),
    #[opcode(0x56)] #[asmstr("LSR")] #[addrmode("zpx")] LogRShfZpX(u8),

    #[opcode(0x2A)] #[asmstr("ROL")] #[addrmode("imp")] LRotAC,
    #[opcode(0x2E)] #[asmstr("ROL")] #[addrmode("abs")] LRotAbs(u16),
    #[opcode(0x3E)] #[asmstr("ROL")] #[addrmode("abx")] LRotAbsX(u16),
    #[opcode(0x26)] #[asmstr("ROL")] #[addrmode("zpm")] LRotZp(u8),
    #[opcode(0x36)] #[asmstr("ROL")] #[addrmode("zpx")] LRotZpX(u8),

    #[opcode(0x6A)] #[asmstr("ROR")] #[addrmode("imp")] RRotAC,
    #[opcode(0x6E)] #[asmstr("ROR")] #[addrmode("abs")] RRotAbs(u16),
    #[opcode(0x7E)] #[asmstr("ROR")] #[addrmode("abx")] RRotAbsX(u16),
    #[opcode(0x66)] #[asmstr("ROR")] #[addrmode("zpm")] RRotZp(u8),
    #[opcode(0x76)] #[asmstr("ROR")] #[addrmode("zpx")] RRotZpX(u8),

    // Logic
    #[opcode(0x29)] #[asmstr("AND")] #[addrmode("imm")] AndImm(u8),
    #[opcode(0x2D)] #[asmstr("AND")] #[addrmode("abs")] AndAbs(u16),
    #[opcode(0x3D)] #[asmstr("AND")] #[addrmode("abx")] AndAbsX(u16),
    #[opcode(0x39)] #[asmstr("AND")] #[addrmode("aby")] AndAbsY(u16),
    #[opcode(0x25)] #[asmstr("AND")] #[addrmode("zpm")] AndZp(u8),
    #[opcode(0x35)] #[asmstr("AND")] #[addrmode("zpx")] AndZpX(u8),
    #[opcode(0x21)] #[asmstr("AND")] #[addrmode("zxi")] AndZpXInd(u8),
    #[opcode(0x31)] #[asmstr("AND")] #[addrmode("zyi")] AndZpYInd(u8),

    #[opcode(0x2C)] #[asmstr("BIT")] #[addrmode("abs")] BitAbs(u16),
    #[opcode(0x24)] #[asmstr("BIT")] #[addrmode("zpm")] BitZp(u8),

    #[opcode(0x49)] #[asmstr("EOR")] #[addrmode("imm")] EorImm(u8),
    #[opcode(0x4D)] #[asmstr("EOR")] #[addrmode("abs")] EorAbs(u16),
    #[opcode(0x5D)] #[asmstr("EOR")] #[addrmode("abx")] EorAbsX(u16),
    #[opcode(0x59)] #[asmstr("EOR")] #[addrmode("aby")] EorAbsY(u16),
    #[opcode(0x45)] #[asmstr("EOR")] #[addrmode("zpm")] EorZp(u8),
    #[opcode(0x55)] #[asmstr("EOR")] #[addrmode("zpx")] EorZpX(u8),
    #[opcode(0x41)] #[asmstr("EOR")] #[addrmode("zxi")] EorZpXInd(u8),
    #[opcode(0x51)] #[asmstr("EOR")] #[addrmode("zyi")] EorZpYInd(u8),

    #[opcode(0x09)] #[asmstr("ORA")] #[addrmode("imm")] OrImm(u8),
    #[opcode(0x0D)] #[asmstr("ORA")] #[addrmode("abs")] OrAbs(u16),
    #[opcode(0x1D)] #[asmstr("ORA")] #[addrmode("abx")] OrAbsX(u16),
    #[opcode(0x19)] #[asmstr("ORA")] #[addrmode("aby")] OrAbsY(u16),
    #[opcode(0x05)] #[asmstr("ORA")] #[addrmode("zpm")] OrZp(u8),
    #[opcode(0x15)] #[asmstr("ORA")] #[addrmode("zpx")] OrZpX(u8),
    #[opcode(0x01)] #[asmstr("ORA")] #[addrmode("zxi")] OrZpXInd(u8),
    #[opcode(0x11)] #[asmstr("ORA")] #[addrmode("zyi")] OrZpYInd(u8),

    // Arithmetic
    #[opcode(0x69)] #[asmstr("ADC")] #[addrmode("imm")] AddImm(u8),
    #[opcode(0x6D)] #[asmstr("ADC")] #[addrmode("abs")] AddAbs(u16),
    #[opcode(0x7D)] #[asmstr("ADC")] #[addrmode("abx")] AddAbsX(u16),
    #[opcode(0x79)] #[asmstr("ADC")] #[addrmode("aby")] AddAbsY(u16),
    #[opcode(0x65)] #[asmstr("ADC")] #[addrmode("zpm")] AddZp(u8),
    #[opcode(0x75)] #[asmstr("ADC")] #[addrmode("zpx")] AddZpX(u8),
    #[opcode(0x61)] #[asmstr("ADC")] #[addrmode("zxi")] AddZpXInd(u8),
    #[opcode(0x71)] #[asmstr("ADC")] #[addrmode("zyi")] AddZpYInd(u8),

    #[opcode(0xC9)] #[asmstr("CMP")] #[addrmode("imm")] CmpACImm(u8),
    #[opcode(0xCD)] #[asmstr("CMP")] #[addrmode("abs")] CmpACAbs(u16),
    #[opcode(0xDD)] #[asmstr("CMP")] #[addrmode("abx")] CmpACAbsX(u16),
    #[opcode(0xD9)] #[asmstr("CMP")] #[addrmode("aby")] CmpACAbsY(u16),
    #[opcode(0xC5)] #[asmstr("CMP")] #[addrmode("zpm")] CmpACZp(u8),
    #[opcode(0xD5)] #[asmstr("CMP")] #[addrmode("zpx")] CmpACZpX(u8),
    #[opcode(0xC1)] #[asmstr("CMP")] #[addrmode("zxi")] CmpACZpXInd(u8),
    #[opcode(0xD1)] #[asmstr("CMP")] #[addrmode("zyi")] CmpACZpYInd(u8),

    #[opcode(0xE0)] #[asmstr("CPX")] #[addrmode("imm")] CmpXImm(u8),
    #[opcode(0xEC)] #[asmstr("CPX")] #[addrmode("abs")] CmpXAbs(u16),
    #[opcode(0xE4)] #[asmstr("CPX")] #[addrmode("zpm")] CmpXZp(u8),

    #[opcode(0xC0)] #[asmstr("CPY")] #[addrmode("imm")] CmpYImm(u8),
    #[opcode(0xCC)] #[asmstr("CPY")] #[addrmode("abs")] CmpYAbs(u16),
    #[opcode(0xC4)] #[asmstr("CPY")] #[addrmode("zpm")] CmpYZp(u8),

    #[opcode(0xE9)] #[asmstr("SBC")] #[addrmode("imm")] SubImm(u8),
    #[opcode(0xED)] #[asmstr("SBC")] #[addrmode("abs")] SubAbs(u16),
    #[opcode(0xFD)] #[asmstr("SBC")] #[addrmode("abx")] SubAbsX(u16),
    #[opcode(0xF9)] #[asmstr("SBC")] #[addrmode("aby")] SubAbsY(u16),
    #[opcode(0xE5)] #[asmstr("SBC")] #[addrmode("zpm")] SubZp(u8),
    #[opcode(0xF5)] #[asmstr("SBC")] #[addrmode("zpx")] SubZpX(u8),
    #[opcode(0xE1)] #[asmstr("SBC")] #[addrmode("zxi")] SubZpXInd(u8),
    #[opcode(0xF1)] #[asmstr("SBC")] #[addrmode("zyi")] SubZpYInd(u8),

    #[opcode(0xCE)] #[asmstr("DEC")] #[addrmode("abs")] DecMemAbs(u16),
    #[opcode(0xDE)] #[asmstr("DEC")] #[addrmode("abx")] DecMemAbsX(u16),
    #[opcode(0xC6)] #[asmstr("DEC")] #[addrmode("zpm")] DecMemZp(u8),
    #[opcode(0xD6)] #[asmstr("DEC")] #[addrmode("zpx")] DecMemZpX(u8),
    #[opcode(0xCA)] #[asmstr("DEX")] #[addrmode("imp")] DecX,
    #[opcode(0x88)] #[asmstr("DEY")] #[addrmode("imp")] DecY,

    #[opcode(0xEE)] #[asmstr("INC")] #[addrmode("abs")] IncMemAbs(u16),
    #[opcode(0xFE)] #[asmstr("INC")] #[addrmode("abx")] IncMemAbsX(u16),
    #[opcode(0xE6)] #[asmstr("INC")] #[addrmode("zpm")] IncMemZp(u8),
    #[opcode(0xF6)] #[asmstr("INC")] #[addrmode("zpx")] IncMemZpX(u8),
    #[opcode(0xE8)] #[asmstr("INX")] #[addrmode("imp")] IncX,
    #[opcode(0xC8)] #[asmstr("INY")] #[addrmode("imp")] IncY,

    // Control
    #[opcode(0x00)] #[asmstr("BRK")] #[addrmode("imp")] Break,

    #[opcode(0x4C)] #[asmstr("JMP")] #[addrmode("abs")] JumpAbs(u16),
    #[opcode(0x6C)] #[asmstr("JMP")] #[addrmode("abi")] JumpAbsInc(u16),
    #[opcode(0x20)] #[asmstr("JSR")] #[addrmode("abs")] JumpSubAbs(u16),

    #[opcode(0x40)] #[asmstr("RTI")] #[addrmode("imp")] RetInt,
    #[opcode(0x60)] #[asmstr("RTS")] #[addrmode("imp")] RetSub,

    // Branch
    #[opcode(0x90)] #[asmstr("BCC")] #[addrmode("rel")] BranchNotCarry(u8),
    #[opcode(0xB0)] #[asmstr("BCS")] #[addrmode("rel")] BranchCarry(u8),
    #[opcode(0xF0)] #[asmstr("BEQ")] #[addrmode("rel")] BranchZero(u8),
    #[opcode(0x30)] #[asmstr("BMI")] #[addrmode("rel")] BranchNeg(u8),
    #[opcode(0xD0)] #[asmstr("BNE")] #[addrmode("rel")] BranchNotZero(u8),
    #[opcode(0x10)] #[asmstr("BPL")] #[addrmode("rel")] BranchNotNeg(u8),
    #[opcode(0x50)] #[asmstr("BVC")] #[addrmode("rel")] BranchNotOver(u8),
    #[opcode(0x70)] #[asmstr("BVS")] #[addrmode("rel")] BranchOver(u8),

    // Flags
    #[opcode(0x18)] #[asmstr("CLC")] #[addrmode("imp")] ClrCarry,
    #[opcode(0xD8)] #[asmstr("CLD")] #[addrmode("imp")] ClrDec,
    #[opcode(0x58)] #[asmstr("CLI")] #[addrmode("imp")] ClrIntDis,
    #[opcode(0xB8)] #[asmstr("CLV")] #[addrmode("imp")] ClrOver,

    #[opcode(0x38)] #[asmstr("SEC")] #[addrmode("imp")] SetCarry,
    #[opcode(0xF8)] #[asmstr("SED")] #[addrmode("imp")] SetDec,
    #[opcode(0x78)] #[asmstr("SEI")] #[addrmode("imp")] SetIntDis,
    
    // Other
    #[opcode(0xEA)] #[asmstr("NOP")] #[addrmode("imp")] NoOp,
    #[opcode(0xF2)] #[asmstr("JAM")] #[addrmode("imp")] Jam,
    #[opcode(0xFF)] #[asmstr("SIG")] #[addrmode("imm")] EmuSignal(u8),
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
            "JumpAbs $12A2".parse::<Instruction>().unwrap(),
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
            "JumpAbs $12A2GH"
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
            "LoadACImm $A2".parse::<Instruction>().unwrap(),
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
            "LoadACImm $12A2GH"
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
            "StoreACZp $A2".parse::<Instruction>().unwrap(),
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
            "StoreACZp $12A2GH"
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
