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
  reg [15:0] RAM  [RAM_BYTES - 1:0];

  /// program counter
  reg [7:0] pc;

  // if enabled, on each clock edge, start reading from ram, and execute the instruction
  reg [7:0] inst;

  // registers
  reg [7:0] r0;
  reg [7:0] r1;
  reg [7:0] r2;
  reg [7:0] r3;

  // initial program counter
  initial begin
    pc   <= 8'b0;
    inst <= 8'b0;

    r0   <= 8'b0;
    r1   <= 8'b0;
    r2   <= 8'b0;
    r3   <= 8'b0;
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
      case (inst[7:5])
        3'b000: r0 <= RAM[inst[4:0]];
        3'b100: RAM[inst[4:0]] <= r0;

        // on other instructions, do nothing
        default: ;
      endcase

      // increment program counter
      pc <= pc + 1;

    end
  end
endmodule

