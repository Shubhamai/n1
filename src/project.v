/*
 * Copyright (c) 2024 Your Name
 * SPDX-License-Identifier: Apache-2.0
 */

`default_nettype none

module tt_um_n1 #(
    parameter RAM_BYTES = 64
) (
    input  wire [7:0] ui_in,    // Dedicated inputs
    output reg [7:0] uo_out,   // Dedicated outputs
    input  wire [7:0] uio_in,   // IOs: Input path
    output wire [7:0] uio_out,  // IOs: Output path
    output wire [7:0] uio_oe,   // IOs: Enable path (active high: 0=input, 1=output)
    input  wire       ena,      // always 1 when the design is powered, so you can ignore it
    input  wire       clk,      // clock
    input  wire       rst_n     // reset_n - low to reset
);

  localparam addr_bits = $clog2(RAM_BYTES);

  wire [addr_bits-1:0] addr = ui_in[addr_bits-1:0];
  wire ram_select = ui_in[6]; // select RAM (0 - program or 1 - data memory)
  wire wr_en = ui_in[7]; // write enable
  assign uio_oe  = 8'b0;  // All bidirectional IOs are inputs
  assign uio_out = 8'b0;

  reg [7:0] PROGRAM_RAM[RAM_BYTES - 1:0];
  reg [7:0] DATA_RAM[RAM_BYTES - 1:0];

  always @(posedge clk) begin
    if (!rst_n) begin
      uo_out <= 8'b0;
      for (int i = 0; i < RAM_BYTES; i++) begin
        PROGRAM_RAM[i] <= 8'b0;
        DATA_RAM[i] <= 8'b0;
      end
    end else begin
      if (wr_en && !ram_select) begin
        PROGRAM_RAM[addr] <= uio_in;
      end
      if (wr_en && ram_select) begin
        DATA_RAM[addr] <= uio_in;
      end
      if (!wr_en && !ram_select) begin
        uo_out <= PROGRAM_RAM[addr];
      end
      if (!wr_en && ram_select) begin
        uo_out <= DATA_RAM[addr];
      end
    end
  end



endmodule

