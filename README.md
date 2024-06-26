![](../../workflows/gds/badge.svg) ![](../../workflows/docs/badge.svg) ![](../../workflows/test/badge.svg) ![](../../workflows/fpga/badge.svg)

# n1

# Introduction

This project implements a very simple custom CPU design in Verilog along with an assembler written in Rust.

### Technical Specifications

the n1 CPU is a 16-bit, [single-issue](https://electronics.stackexchange.com/a/145477), [[in-order] (scalar)](https://en.wikipedia.org/wiki/Scalar_processor) processor with a [Harvard architecture](https://en.wikipedia.org/wiki/Harvard_architecture), meaning it has separate memory for instructions and data. Key specifications include:

- [Word size](https://en.wikipedia.org/wiki/Word_(computer_architecture)): 16 bits
- Instruction size: 16 bits
- Register file: 4 general-purpose registers (4 bits each)
- 128 bytes of RAM
- Arithmetic Logic Unit (ALU) supporting basic operations
- Control unit for instruction decoding and execution
- Simple branching and function call capabilities
- Pipeline stages: None (single-cycle execution)
- Floating-point unit: Not included

## Table of Contents

1. [CPU Implementation](#cpu-implementation)
2. [Instruction Set Architecture (ISA)](#instruction-set-architecture-isa)
3. [Assembler](#assembler)
4. [How to Use This Project](#how-to-use-this-project)
5. [Example Code](#example-code)
6. [Limitations and Shortcomings](#limitations-and-shortcomings)

## CPU Implementation

The CPU is implemented in Verilog ([`./src/project.v`](./src/project.v)). Here's a brief overview of its components:

1. **Registers**: The CPU has 4 general-purpose registers, a program counter, and a stack pointer.
2. **ALU**: Performs arithmetic and logical operations (ADD, SUB, MUL, DIV).
3. **Control Unit**: Decodes instructions and controls the datapath.
4. **Memory**: 128 bytes of RAM for data storage.
5. **Instruction Fetch**: Reads instructions from memory based on the program counter.
6. **Instruction Decode**: Interprets the opcode and operands.
7. **Instruction Execute**: Performs the operation specified by the instruction.

The CPU follows a simple fetch-decode-execute cycle:

1. Fetch the instruction from memory at the address specified by the program counter.
2. Decode the instruction to determine the operation and operands.
3. Execute the instruction, which may involve reading/writing registers, performing ALU operations, or updating the program counter for branches.
4. Update the program counter to the next instruction (unless a branch occurred).

## Instruction Set Architecture (ISA)

Our custom ISA uses 16-bit instructions. Here's a table of the supported instructions, you can find the full list in `assembler/enums.rs`:

| Instruction | Opcode (4 bits) | Operands     | Description                   |
| ----------- | --------------- | ------------ | ----------------------------- |
| MOV         | 0001            | Rd, Imm      | Move immediate to register    |
| STORE       | 0010            | Rs, Addr     | Store register to memory      |
| ADD         | 0011            | Rd, Rs1, Rs2 | Add two registers             |
| SUB         | 0100            | Rd, Rs1, Rs2 | Subtract two registers        |
| MUL         | 0101            | Rd, Rs1, Rs2 | Multiply two registers        |
| DIV         | 0110            | Rd, Rs1, Rs2 | Divide two registers          |
| PRINT       | 0111            | Addr         | Print value at memory address |
| END         | 1000            | -            | End program                   |
| CMP         | 1001            | Rs1, Rs2     | Compare two registers         |
| JMP         | 1010            | Addr         | Unconditional jump            |
| JNE         | 1011            | Addr         | Jump if not equal             |
| JLE         | 1100            | Addr         | Jump if less or equal         |
| CALL        | 1101            | Addr         | Call function                 |
| RET         | 1110            | -            | Return from function          |

## Assembler

The assembler, written in Rust, converts assembly language code into machine code that can be executed by our custom CPU. It consists of three main components:

1. [**Lexer** ](./assembler/lexer.rs): Tokenizes the input assembly code.
2. [**Parser** ](./assembler/parser.rs): Converts tokens into `Instruction` enum variants.
3. [**Code Generator**](./assembler/main.rs): Converts `Instruction` enums into binary machine code.

The assembler process:

1. Read the input assembly file.
2. Tokenize the input using the lexer.
3. Parse the tokens into `Instruction` enum variants.
4. Resolve labels and calculate jump addresses.
5. Generate binary machine code for each instruction.
6. Write the resulting machine code to an output file.

## How to Use This Project

1. **Build the CPU**:

   - The project is built using tinytapeout template. To run an example machine code, simply
   - Navigate to the `./test` directory.
   - Run the simulation: `make -B`

2. **Use the Assembler**:

   - Ensure you have Rust installed.
   - Navigate to the assembler directory.
   - Build the assembler: `cargo run ./examples/add_print.asm`

3. **Run Your Program**:
   - Write your assembly code in a `.asm` file.
   - Assemble it using the assembler.
   - Load the resulting machine code into the CPU's memory.
   - Run the CPU simulation.

## Example Code

Here's a simple example of assembly from [`./examples/add_print.asm`](./examples/add_print.asm) code that calculates and prints the sum of two numbers:

```assembly
main:
.entry main

main:
    mov r3 #1
    mov r2 #2
    jump +3
    mov r3 #14
    mov r2 #6
    add r1 r2 r3
    store 0x64 r1
    print 0x64

    end
```

To run this example:

1. Save it as `add_print.asm` in the `./examples` directory.
2. Assemble it: `cargo run ./examples/add_print.asm`
3. Navigate to the `./test` directory.
4. Run the simulation: `make -B`

This will calculate and print the sum of 1 and 2, which is 3.

---

## Improvements

The n1 CPU is a simple design meant for educational purposes. Here are some potential improvements to make it more practical:

1. **Limited Instruction Set**: The CPU supports only basic arithmetic, memory, and control flow operations. It lacks more advanced instructions found in modern CPUs, such as bitwise operations, floating-point arithmetic, or SIMD instructions.

2. **Small Address Space**: With only 8 bits for addressing, the CPU can only access 256 memory locations, severely limiting the size of programs and data that can be processed.

3. **Limited Register File**: The CPU has only 4 general-purpose registers, which may lead to frequent memory accesses and potential performance bottlenecks.

4. **No Pipelining**: The single-cycle execution model, while simple to understand, results in lower performance compared to pipelined architectures.

5. **Limited Data Types**: The CPU only supports 16-bit integer operations. There's no native support for smaller (e.g., bytes) or larger (e.g., 32-bit) data types, or for floating-point numbers.
