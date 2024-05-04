#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl std::str::FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r0" => Ok(Register::R0),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum Operand {
    Registers(Register),
    Immediate(ImmediateBits),
    MemoryAddress(MemoryAddressBits),
}

type ImmediateBits = u16; // 8 bits
type MemoryAddressBits = u16; // 8 bits
type RegisterBits = u16; // 3 bits
type OpcodeBits = u16; // 4 bits

#[derive(Debug)]
#[repr(u16)]
pub enum Instruction {
    // register and immediate
    Mov {
        reg: Register,
        imm: ImmediateBits,
    },
    Store {
        // memory address
        addr: MemoryAddressBits,
        // register
        reg: Register,
    },
    // Load(Operand, Operand) = 1,
    Add {
        // destination register
        dest: Register,
        // source register 1
        src1: Register,
        // source register 2
        src2: Register,
    },
    Print {
        // memory address
        addr: MemoryAddressBits,
    },
    End,
}

// to binary
impl Instruction {
    fn get_opcode(&self) -> OpcodeBits {
        match self {
            Instruction::Mov { .. } => 1,
            Instruction::Store { .. } => 2,
            Instruction::Add { .. } => 3,
            Instruction::Print { .. } => 4,
            Instruction::End => 5,
        }
    }

    pub fn to_binary(&self) -> u16 {
        match self {
            Instruction::Mov { reg, imm } => {
                let reg = *reg as RegisterBits;
                let imm = *imm as ImmediateBits;

                let opcode: u16 = self.get_opcode() as u16;
                opcode << 12 | reg << 9 | imm
            }
            Instruction::Store { addr, reg } => {
                let addr = *addr as u16;
                let reg = *reg as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | reg << 9 | addr
            }
            Instruction::Add { dest, src1, src2 } => {
                let dest = *dest as u16;
                let src1 = *src1 as u16;
                let src2 = *src2 as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | dest << 9 | src1 << 7 | src2
            }
            Instruction::Print { addr } => {
                let addr = *addr as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | addr
            }
            Instruction::End => {
                let opcode = self.get_opcode() as u16;
                opcode << 12
            }
        }
    }
}
