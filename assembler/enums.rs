type RegisterBits = String; // 4-bits
type ImmediateBits = String; // 8 bits
type MemoryAddressBits = String; // 8 bits
type OpcodeBits = String; // 4 bits

#[derive(Debug)]
#[repr(u16)]
pub enum Instruction {
    // register and immediate
    Mov(RegisterBits, ImmediateBits),
    Store(RegisterBits, MemoryAddressBits),
    Add(RegisterBits, RegisterBits, RegisterBits),
    Sub(RegisterBits, RegisterBits, RegisterBits),
    Mul(RegisterBits, RegisterBits, RegisterBits),
    Div(RegisterBits, RegisterBits, RegisterBits),
    Print(MemoryAddressBits),

    Compare(RegisterBits, RegisterBits),
    Jump(MemoryAddressBits),
    JumpNotEqual(MemoryAddressBits),
    JumpLessEqual(MemoryAddressBits),
    EntryFunction(MemoryAddressBits),

    End,
}

impl Instruction {
    pub fn to_binary(&self) -> String {
        match self {
            Instruction::Mov(r, imm) => format!("0001{}{}{}", r, "0", imm),
            Instruction::Store(r, addr) => format!("0010{}{}{}", r, "0", addr),
            // NOTE: 001 signifies the output is register, probably not needed
            Instruction::Add(r1, r2, r3) => format!("0011{}{}{}{}", r1, r2, r3, "001"),
            Instruction::Sub(r1, r2, r3) => format!("0100{}{}{}{}", r1, r2, r3, "001"),
            Instruction::Mul(r1, r2, r3) => format!("0101{}{}{}{}", r1, r2, r3, "001"),
            Instruction::Div(r1, r2, r3) => format!("0110{}{}{}{}", r1, r2, r3, "001"),
            Instruction::Print(addr) => format!("0111{}{}", "0000", addr),
            Instruction::Compare(r1, r2) => format!("1001{}{}000000", r1, r2),
            Instruction::Jump(addr) => format!("1010{}{}", "0000", addr),
            Instruction::JumpNotEqual(addr) => format!("1011{}{}", "0000", addr),
            Instruction::JumpLessEqual(addr) => format!("1100{}{}", "0000", addr),
            Instruction::EntryFunction(addr) => todo!(),
            Instruction::End => "1000000000000000".to_string(),
        }
    }
}
