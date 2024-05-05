/*
 * Copyright (c) 2024 Your Name
 * SPDX-License-Identifier: Apache-2.0
 */

`default_nettype none

module tt_um_n1 #(
    parameter RAM_BYTES = 128
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
  //   $readmemb("example.asm.txt", RAM);
  // end

  /// program counter
  reg [15:0] pc;

  // if enabled, on each clock edge, start reading from ram, and execute the instruction
  reg [15:0] inst;

  // 4 general purpose registers
  reg [15:0] registers[3:0];


  // initial program counter
  initial begin
    pc <= 8'b0;
    inst <= 8'b0;

    // initialize registers
    registers[0] <= 8'b0;
    registers[1] <= 8'b0;
    registers[2] <= 8'b0;
    registers[3] <= 8'b0;
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
      inst <= RAM[pc];

      // execute instruction , first 5 bits are location of the memory, next 3 bits are the operation
      case (inst[15:12])

        // move from immediate to register
        4'b0001: begin
          $display("moving immediate: %b to register: %b", inst[7:0], inst[11:9]);
          registers[inst[11:9]] <= inst[7:0];
        end

        // store from register to memory
        4'b0010: begin
          $display("storing from register: %b to memory address: %b", inst[11:9], inst[7:0]);
          RAM[inst[7:0]] <= registers[inst[11:9]];
        end

        // print memory address value, assing to register uo_out
        4'b0111: begin
          $display("memory address: %b, value: %b", inst[7:0], RAM[inst[7:0]]);
          uo_out <= RAM[inst[7:0]];
        end

        // add
        4'b0011: begin
          $display("adding register: %b + register: %b to register: %b", inst[11:9], inst[8:6],
                   inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] + registers[inst[5:3]];
        end

        // sub
        4'b0100: begin
          $display("subtracting register: %b - register: %b to register: %b", inst[11:9],
                   inst[8:6], inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] - registers[inst[5:3]];
        end

        // mul
        4'b0101: begin
          $display("multiplying register: %b * register: %b to register: %b", inst[11:9],
                   inst[8:6], inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] * registers[inst[5:3]];
        end

        // div - TODO division by zero
        4'b0110: begin
          $display("dividing register: %b / register: %b to register: %b", inst[11:9], inst[8:6],
                   inst[5:3]);
          registers[inst[11:9]] <= registers[inst[8:6]] / registers[inst[5:3]];
        end

        // end of program
        4'b1000: begin
          $display("end of program");
          // $finish;
        end

        default: ;
      endcase

      // increment program counter
      pc <= pc + 1;

    end
  end
endmodule

