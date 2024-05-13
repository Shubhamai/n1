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
pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(ImmediateBits),
}

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
        src2: RegisterOrImmediate,
    },
    Sub {
        // destination register
        dest: Register,
        // source register 1
        src1: Register,
        // source register 2
        src2: RegisterOrImmediate,
    },
    Mul {
        // destination register
        dest: Register,
        // source register 1
        src1: Register,
        // source register 2
        src2: RegisterOrImmediate,
    },
    Div {
        // destination register
        dest: Register,
        // source register 1
        src1: Register,
        // source register 2
        src2: RegisterOrImmediate,
    },
    Compare {
        // source register 1
        src1: Register,
        // source register 2
        src2: Register,
    },
    Jump {
        // memory address
        addr: MemoryAddressBits,
    },
    JumpNotEqual {
        // memory address
        addr: MemoryAddressBits,
    },
    JumpLessEqual {
        // memory address
        addr: MemoryAddressBits,
    },
    EntryFunction {
        // memory address
        addr: MemoryAddressBits,
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
            Instruction::Sub { .. } => 4,
            Instruction::Mul { .. } => 5,
            Instruction::Div { .. } => 6,
            Instruction::Print { .. } => 7,
            Instruction::End => 8,

            Instruction::Compare { .. } => 9,
            Instruction::Jump { .. } => 10,
            Instruction::JumpNotEqual { .. } => 11,
            Instruction::JumpLessEqual { .. } => 12,
            Instruction::EntryFunction { .. } => 13,
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
            // Instruction::Add { dest, src1, src2 } => {
            //     let dest = *dest as u16;
            //     let src1 = *src1 as u16;

            //     let is_register = match src2 {
            //         RegisterOrImmediate::Register(_) => true,
            //         RegisterOrImmediate::Immediate(_) => false,
            //     };
            //     let src2 = match src2 {
            //         RegisterOrImmediate::Register(reg) => *reg as u16,
            //         RegisterOrImmediate::Immediate(imm) => *imm as u16,
            //     };

            //     let opcode = self.get_opcode() as u16;

            //     // 4 bits opcode, 3 bits dest, 3 bits src1, 3 bits src2, ins[0] = 0 for register, 1 for immediate
            //     opcode << 12 | dest << 9 | src1 << 6 | src2 << 3 | is_register as u16

            // }
            // Instruction::Sub { dest, src1, src2 } => {
            //     let dest = *dest as u16;
            //     let src1 = *src1 as u16;
            //     let is_register = match src2 {
            //         RegisterOrImmediate::Register(_) => true,
            //         RegisterOrImmediate::Immediate(_) => false,
            //     };
            //     let src2 = match src2 {
            //         RegisterOrImmediate::Register(reg) => *reg as u16,
            //         RegisterOrImmediate::Immediate(imm) => *imm as u16,
            //     };

            //     let opcode = self.get_opcode() as u16;

            //     // 4 bits opcode, 3 bits dest, 3 bits src1, 3 bits src2, ins[0] = 0 for register, 1 for immediate
            //     opcode << 12 | dest << 9 | src1 << 6 | src2 << 3 | is_register as u16
            // }
            // Instruction::Mul { dest, src1, src2 } => {
            //     let dest = *dest as u16;
            //     let src1 = *src1 as u16;
            //     let is_register = match src2 {
            //         RegisterOrImmediate::Register(_) => true,
            //         RegisterOrImmediate::Immediate(_) => false,
            //     };
            //     let src2 = match src2 {
            //         RegisterOrImmediate::Register(reg) => *reg as u16,
            //         RegisterOrImmediate::Immediate(imm) => *imm as u16,
            //     };

            //     let opcode = self.get_opcode() as u16;

            //     // 4 bits opcode, 3 bits dest, 3 bits src1, 3 bits src2, ins[0] = 0 for register, 1 for immediate
            //     opcode << 12 | dest << 9 | src1 << 6 | src2 << 3 | is_register as u16
            // }
            // Instruction::Div { dest, src1, src2 } => {
            //     let dest = *dest as u16;
            //     let src1 = *src1 as u16;
            //     let is_register = match src2 {
            //         RegisterOrImmediate::Register(_) => true,
            //         RegisterOrImmediate::Immediate(_) => false,
            //     };
            //     let src2 = match src2 {
            //         RegisterOrImmediate::Register(reg) => *reg as u16,
            //         RegisterOrImmediate::Immediate(imm) => *imm as u16,
            //     };

            //     let opcode = self.get_opcode() as u16;

            //     // 4 bits opcode, 3 bits dest, 3 bits src1, 3 bits src2, ins[0] = 0 for register, 1 for immediate
            //     opcode << 12 | dest << 9 | src1 << 6 | src2 << 3 | is_register as u16
            // }
            Instruction::Add { dest, src1, src2 }
            | Instruction::Sub { dest, src1, src2 }
            | Instruction::Mul { dest, src1, src2 }
            | Instruction::Div { dest, src1, src2 } => {
                let dest = *dest as u16;
                let src1 = *src1 as u16;

                let is_register = match src2 {
                    RegisterOrImmediate::Register(_) => true,
                    RegisterOrImmediate::Immediate(_) => false,
                };
                let src2 = match src2 {
                    RegisterOrImmediate::Register(reg) => *reg as u16,
                    RegisterOrImmediate::Immediate(imm) => *imm as u16,
                };

                let opcode = self.get_opcode() as u16;

                // 4 bits opcode, 3 bits dest, 3 bits src1, 3 bits src2, ins[0] = 0 for register, 1 for immediate
                opcode << 12 | dest << 9 | src1 << 6 | src2 << 3 | is_register as u16
            }

            Instruction::Compare { src1, src2 } => {
                let src1 = *src1 as u16;
                let src2 = *src2 as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | src1 << 9 | src2 << 6
            }
            // Instruction::Jump { addr } => {
            //     let addr = *addr as u16;
            //     let opcode = self.get_opcode() as u16;
            //     opcode << 12 | addr
            // }
            // Instruction::JumpNotEqual { addr } => {
            //     let addr = *addr as u16;
            //     let opcode = self.get_opcode() as u16;
            //     opcode << 12 | addr
            // }
            Instruction::Jump { addr }
            | Instruction::JumpNotEqual { addr }
            | Instruction::JumpLessEqual { addr } => {
                let addr = *addr as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | addr
            }

            Instruction::Print { addr } => {
                let addr = *addr as u16;
                let opcode = self.get_opcode() as u16;
                opcode << 12 | addr
            }
            Instruction::EntryFunction { addr } => {
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
