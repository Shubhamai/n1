/*
 * Copyright (c) 2024 Your Name
 * SPDX-License-Identifier: Apache-2.0
 */

`default_nettype none

module tt_um_n1 #(
    parameter RAM_SIZE = 128  // 128 bytes of RAM
) (
    input  wire [7:0] ui_in,    // Dedicated inputs
    output reg  [7:0] uo_out,   // Dedicated outputs
    input  wire [7:0] uio_in,   // IOs: Input path
    output wire [7:0] uio_out,  // IOs: Output path
    output wire [7:0] uio_oe,   // IOs: Enable path (active high: 0=input, 1=output)
    input  wire       ena,      // Enable signal (always 1 when the design is powered)
    input  wire       clk,      // Clock signal
    input  wire       rst_n     // Reset signal (active low)
);

  // Calculate the number of bits needed to address the RAM
  localparam ADDR_BITS = $clog2(RAM_SIZE);


  // Input signals
  wire [ADDR_BITS-1:0] ram_addr = ui_in[ADDR_BITS-1:0];
  wire write_enable = ui_in[7];

  // Set all bidirectional IOs as inputs
  assign uio_oe  = 8'b0;
  assign uio_out = 8'b0;

  // RAM declaration
  reg [15:0] ram[RAM_SIZE - 1:0];

  // load the RAM from a file - TODO for simulation only
  initial begin
    $readmemb("../examples/functions.asm.txt", ram);
  end

  // CPU Registers
  reg [15:0] program_counter;
  reg [15:0] instruction;
  reg [15:0] stack_pointer;
  reg [15:0] general_registers[3:0];

  // Stack
  reg [15:0] stack[31:0];

  // CPSR (Current Program Status Register) flags
  reg negative_flag;
  reg zero_flag;
  reg carry_flag;
  reg overflow_flag;

  // Initialize CPU state
  initial begin
    program_counter = 16'b0;
    instruction = 16'b0;
    stack_pointer = 16'b0;
    
    for (int i = 0; i < 4; i = i + 1) begin
      general_registers[i] = 16'b0;
    end

    negative_flag = 1'b0;
    zero_flag = 1'b0;
    carry_flag = 1'b0;
    overflow_flag = 1'b0;
  end

  // Main CPU logic
  always @(posedge clk) begin
    if (!rst_n) begin
      // Reset state
      uo_out <= 8'b0;
      
      // Write to memory only if write enable is high during reset
      if (write_enable) begin
        ram[ram_addr] <= uio_in;
      end
      uo_out <= ram[ram_addr];
    end else begin
      // Fetch instruction
      instruction <= ram[program_counter];
  
      // Decode and execute instruction
      case (instruction[15:12])
        4'b0001: execute_move_immediate();
        4'b0010: execute_store_to_memory();
        4'b0011: execute_add();
        4'b0100: execute_subtract();
        4'b0101: execute_multiply();
        4'b0110: execute_divide();
        4'b0111: execute_print_memory();
        4'b1000: ; // End of program (no operation)
        4'b1001: execute_compare();
        4'b1010: execute_jump();
        4'b1011: execute_jump_not_equal();
        4'b1100: execute_jump_less_equal();
        4'b1101: execute_call();
        4'b1110: execute_return();
        default: $display("Unknown instruction: %b", instruction[15:12]);
      endcase
    end
  end

  // Instruction execution tasks
  task execute_move_immediate;
    begin
      $display("Moving immediate: %b to register: %b", instruction[7:0], instruction[11:9]);
      general_registers[instruction[11:9]] <= instruction[7:0];
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_store_to_memory;
    begin
      $display("Storing from register: %b to memory address: %b", instruction[11:9], instruction[7:0]);
      ram[instruction[7:0]] <= general_registers[instruction[11:9]];
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_add;
    begin
      $display("Adding register: %b + register: %b to register: %b", instruction[11:9], instruction[8:6], instruction[5:3]);
      {carry_flag, general_registers[instruction[11:9]]} <= general_registers[instruction[8:6]] + general_registers[instruction[5:3]];
      update_flags(general_registers[instruction[11:9]]);
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_subtract;
    begin
      $display("Subtracting register: %b - register: %b to register: %b", instruction[11:9], instruction[8:6], instruction[5:3]);
      {carry_flag, general_registers[instruction[11:9]]} <= general_registers[instruction[8:6]] - general_registers[instruction[5:3]];
      update_flags(general_registers[instruction[11:9]]);
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_multiply;
    begin
      $display("Multiplying register: %b * register: %b to register: %b", instruction[11:9], instruction[8:6], instruction[5:3]);
      general_registers[instruction[11:9]] <= general_registers[instruction[8:6]] * general_registers[instruction[5:3]];
      update_flags(general_registers[instruction[11:9]]);
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_divide;
    begin
      $display("Dividing register: %b / register: %b to register: %b", instruction[11:9], instruction[8:6], instruction[5:3]);
      if (general_registers[instruction[5:3]] != 0) begin
        general_registers[instruction[11:9]] <= general_registers[instruction[8:6]] / general_registers[instruction[5:3]];
        update_flags(general_registers[instruction[11:9]]);
      end else begin
        $display("Error: Division by zero");
      end
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_print_memory;
    begin
      $display("Memory address: %b, value: %b", instruction[7:0], ram[instruction[7:0]]);
      uo_out <= ram[instruction[7:0]];
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_compare;
    reg [16:0] result;
    begin
      $display("Comparing register: %b (%d) with register: %b (%d)", instruction[11:9], general_registers[instruction[11:9]], instruction[8:6], general_registers[instruction[8:6]]);
      result = general_registers[instruction[11:9]] - general_registers[instruction[8:6]];
      zero_flag <= (result == 0);
      negative_flag <= result[15];
      carry_flag <= !result[16];
      overflow_flag <= (general_registers[instruction[11:9]][15] != general_registers[instruction[8:6]][15]) && (result[15] != general_registers[instruction[11:9]][15]);
      program_counter <= program_counter + 1;
    end
  endtask

  task execute_jump;
    begin
      $display("Jumping to address: %b", instruction[7:0]);
      program_counter <= instruction[7:0];
    end
  endtask

  task execute_jump_not_equal;
    begin
      if (!zero_flag) begin
        $display("Jumping to address: %b", instruction[7:0]);
        program_counter <= instruction[7:0];
      end else begin
        $display("Not jumping, continuing to next instruction");
        program_counter <= program_counter + 1;
      end
    end
  endtask

  task execute_jump_less_equal;
    begin
      if (negative_flag || zero_flag) begin
        $display("Jumping to address: %b", instruction[7:0]);
        program_counter <= instruction[7:0];
      end else begin
        $display("Not jumping, continuing to next instruction");
        program_counter <= program_counter + 1;
      end
    end
  endtask

  task execute_call;
    begin
      $display("Calling address: %b", instruction[7:0]);
      if (stack_pointer < 32) begin
        stack[stack_pointer] <= program_counter + 1;
        stack_pointer <= stack_pointer + 1;
        program_counter <= instruction[7:0];
      end else begin
        $display("Error: Stack overflow");
        program_counter <= program_counter + 1;
      end
    end
  endtask

  task execute_return;
    begin
      if (stack_pointer > 0) begin
        stack_pointer <= stack_pointer - 1;
        program_counter <= stack[stack_pointer - 1];
        $display("Returning to address: %b", stack[stack_pointer - 1]);
      end else begin
        $display("Error: Stack underflow");
        program_counter <= program_counter + 1;
      end
    end
  endtask

  // Helper function to update flags
  function void update_flags(input [15:0] result);
    begin
      negative_flag = result[15];
      zero_flag = (result == 0);
    end
  endfunction

endmodule