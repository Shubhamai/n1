# SPDX-FileCopyrightText: © 2024 Tiny Tapeout
# SPDX-License-Identifier: MIT

import cocotb
from cocotb.clock import Clock
from cocotb.triggers import ClockCycles

# Define the write enable bit
WE = 1 << 7

@cocotb.test()
async def test_project(dut):
    dut._log.info("Start")

    # Set the clock period to 10 us (100 KHz)
    clock = Clock(dut.clk, 10, units="us")
    cocotb.start_soon(clock.start())

    # Reset
    dut._log.info("Reset - Testing Program Memory")
    dut.ena.value = 1
    dut.ui_in.value = 0
    dut.uio_in.value = 0
    # dut.rst_n.value = 0
    # await ClockCycles(dut.clk, 2)

    dut.rst_n.value = 1

    # All the bidirectional ports are used for the data_in signal, so they should be inputs
    # assert int(dut.uio_oe .value) == 0

    await ClockCycles(dut.clk, 1000)

    dut._log.info("read back the bytes and verify they are correct")
    assert dut.uo_out.value == 8

    dut._log.info("all good!")
