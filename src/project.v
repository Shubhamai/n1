/*
 * Copyright (c) 2024 Your Name
 * SPDX-License-Identifier: Apache-2.0
 */

`default_nettype none

module tt_um_n1 #(
    parameter RAM_BYTES = 255  // 255 bytes of RAM
) (
    input  wire [7:0] ui_in,    // Dedicated inputs
    output reg  [7:0] uo_out,   // Dedicated outputs
    input  wire [7:0] uio_in,   // IOs: Input path
    output wire [7:0] uio_out,  // IOs: Output path
    output wire [7:0] uio_oe,   // IOs: Enable path (active high: 0=input, 1=output)
    input  wire       ena,      // always 1 when the design is powered, so you can ignore it
    input  wire       clk,      // clock
    input  wire       rst_n     // reset_n - low to reset
);

  localparam addr_bits = $clog2(RAM_BYTES);

  wire [addr_bits-1:0] addr = ui_in[addr_bits-1:0];
  wire wr_en = ui_in[7];  // write enable
  assign uio_oe  = 8'b0;  // All bidirectional IOs are inputs
  assign uio_out = 8'b0;

  // RAM
  reg [15:0] RAM[RAM_BYTES - 1:0];

  // load the RAM from a file - TODO for simulation only
  // initial begin
  //   $readmemb("loops.asm.txt", RAM);
  // end

  // display the RAM contents
  // initial begin
  //   $display("RAM contents:");
  //   for (int i = 0; i < RAM_BYTES; i = i + 1) begin
  //     $display("RAM[%d] = %b", i, RAM[i]);
  //   end
  // end

  /// program counter
  reg [15:0] pc;

  // if enabled, on each clock edge, start reading from ram, and execute the instruction
  reg [15:0] inst;

  // Stack Pointer (sp) register
  reg [15:0] sp;

  // Global Pointer (gp) register
  reg [15:0] gp;


  // 4 general purpose registers
  reg [15:0] registers[3:0];

  // CPSR flags
  reg N;  // Negative flag
  reg Z;  // Zero flag
  reg C;  // Carry flag
  reg V;  // Overflow flag


  // initial program counter
  initial begin
    pc <= 8'b0;
    inst <= 8'b0;

    sp <= 8'b0;
    gp <= 8'b0;

    // initialize registers
    registers[0] <= 8'b0;
    registers[1] <= 8'b0;
    registers[2] <= 8'b0;
    registers[3] <= 8'b0;

    N <= 1'b0;
    Z <= 1'b0;
    C <= 1'b0;
    V <= 1'b0;
  end

  always @(posedge clk) begin
    if (!rst_n) begin
      uo_out <= 8'b0;

      // write to memory only if write enable is high and in reset state
      if (wr_en) begin
        RAM[addr] <= uio_in;
      end
      uo_out <= RAM[addr];

    end else begin
      // load instruction 
      // $display("loading instruction from address: %b", pc);
      inst <= RAM[pc];

      // execute instruction , first 5 bits are location of the memory, next 3 bits are the operation
      case (inst[15:12])

        // move from immediate to register
        4'b0001: begin
          $display("moving immediate: %b to register: %b", inst[7:0], inst[11:9]);
          registers[inst[11:9]] <= inst[7:0];

          pc <= pc + 1;
        end

        // store from register to memory
        4'b0010: begin
          $display("storing from register: %b to memory address: %b", inst[11:9], inst[7:0]);
          RAM[inst[7:0]] <= registers[inst[11:9]];

          pc <= pc + 1;
        end

        // print memory address value, assing to register uo_out
        4'b0111: begin
          $display("memory address: %b, value: %b", inst[7:0], RAM[inst[7:0]]);
          uo_out <= RAM[inst[7:0]];

          pc <= pc + 1;
        end

        // add                 
        4'b0011: begin
          $display("adding register: %b + register: %b to register: %b", inst[11:9], inst[8:6],
                   inst[5:3]);

          // if inst[0] is 1, then inst[5:3] is immediate value else it is register value


          {C, registers[inst[11:9]]} <= registers[inst[8:6]] + registers[inst[5:3]];
          N <= registers[inst[11:9]][15];
          Z <= (registers[inst[11:9]] == 0);
          V <= (registers[inst[8:6]][15] == registers[inst[5:3]][15]) && (registers[inst[11:9]][15] != registers[inst[8:6]][15]);

          pc <= pc + 1;
        end

        // sub
        4'b0100: begin
          $display("subtracting register: %b - register: %b to register: %b", inst[11:9],
                   inst[8:6], inst[5:3]);
          {C, registers[inst[11:9]]} <= registers[inst[8:6]] - registers[inst[5:3]];
          N <= registers[inst[11:9]][15];
          Z <= (registers[inst[11:9]] == 0);
          V <= (registers[inst[8:6]][15] != registers[inst[5:3]][15]) && (registers[inst[11:9]][15] != registers[inst[8:6]][15]);

          pc <= pc + 1;
        end

        // mul
        4'b0101: begin
          $display("multiplying register: %b * register: %b to register: %b", inst[11:9],
                   inst[8:6], inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] * registers[inst[5:3]];

          pc <= pc + 1;
        end

        // div - TODO division by zero
        4'b0110: begin
          $display("dividing register: %b / register: %b to register: %b", inst[11:9], inst[8:6],
                   inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] / registers[inst[5:3]];

          pc <= pc + 1;
        end


        // cmp
        4'b1001: begin

          // display register with value in decimal
          $display("comparing register: %b (%d) with register: %b (%d)", inst[11:9],
                   registers[inst[11:9]], inst[8:6], registers[inst[8:6]]);

          // {C, Z} <= registers[inst[11:9]] - registers[inst[8:6]];

          Z <= registers[inst[11:9]] == registers[inst[8:6]];
          N <= registers[inst[11:9]] < registers[inst[8:6]];

          
          // N <= registers[inst[11:9]][15] ^ registers[inst[8:6]][15];
          
          // V <= (registers[inst[11:9]][15] != registers[inst[8:6]][15]) && (Z[15] != registers[inst[11:9]][15]);

          // $display("[cmp] Found N: %b, Z: %b, C: %b", N, Z, C);

          pc <= pc + 1;
        end

        // jumpne
        4'b1011: begin
          if (!Z) begin
            $display("[jumpne] jumping to address: %b", inst[7:0]);
            pc <= inst[7:0];
          end else begin
            $display("[jumpne] NOT jumping to address: %b", inst[7:0]);
            pc <= pc + 1;

          end
        end

        // 1100 - jumple - jump if less than or equal
        4'b1100: begin
          $display("[jumple] Found N: %b, Z: %b, C: %b", N, Z, C);

          if (N || Z) begin
            $display("[jumple] jumping to address: %b", inst[7:0]);
            pc <= inst[7:0];
          end else begin
            $display("[jumple] NOT jumping to address: %b", inst[7:0]);
            pc <= pc + 1;

          end
        end

        // jump
        4'b1010: begin
          $display("[jump] jumping to address: %b", inst[7:0]);
          pc <= inst[7:0];
        end

        // end of program
        4'b1000: begin
          $display("end of program");
          // $finish;
        end

        // default: $display("unknown instruction: %b", inst[15:12]);

      endcase


    end
  end
endmodule

