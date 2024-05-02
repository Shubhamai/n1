/*
 * Copyright (c) 2024 Your Name
 * SPDX-License-Identifier: Apache-2.0
 */

`default_nettype none

module tt_um_n1 (
    input  wire [7:0] ui_in,    // Dedicated inputs
    output wire [7:0] uo_out,   // Dedicated outputs
    input  wire [7:0] uio_in,   // IOs: Input path
    output wire [7:0] uio_out,  // IOs: Output path
    output wire [7:0] uio_oe,   // IOs: Enable path (active high: 0=input, 1=output)
    input  wire       ena,      // always 1 when the design is powered, so you can ignore it
    input  wire       clk,      // clock
    input  wire       rst_n     // reset_n - low to reset
);


  // implement data ram - 256 words - 8 bits each
  reg [7:0] ram [0:255];

  // implement program ram - 256 words - 16 bits each
  reg [15:0] pram [0:255];

  /// program counter
  reg [7:0] pc;

  // if enabled, on each clock edge, start reading from program memory, and execute the instruction
  reg [15:0] inst;

  // add pc and data ( test ) ( only first 8 bits )
  reg [7:0] out;


  // load 1 to ram[0] and 2 to ram[1]
  initial begin
    ram[0] <= 8'h01;
    ram[1] <= 8'h02;
  end

  // load 16 bit add instruction to pram
  initial begin
    pram[0] <= 16'h0001;
  end
  
  always @(posedge clk or negedge rst_n) begin

    if (~rst_n) begin
      pc <= 8'h00;
    end else begin
      if (ena) begin
        pc <= pc + 1;

        // read instruction
        inst <= pram[pc];

        // if instruction is 0001, add ram[0] and ram[1]
        if (inst == 16'h0001) begin
          out <= ram[0] + ram[1];
        end


      end
    end
  end  


  assign uo_out  = out;
  assign uio_out = 0;
  assign uio_oe  = 0;

endmodule

