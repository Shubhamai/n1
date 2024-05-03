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


    # write to program memory - 0 or data memory - 1
    SELECT = 0 << 6
    
    # Reset
    dut._log.info("Reset - Testing Program Memory")
    dut.ena.value = 1
    dut.ui_in.value = SELECT | 0
    dut.uio_in.value = 0
    dut.rst_n.value = 0
    await ClockCycles(dut.clk, 10)
    dut.rst_n.value = 1

    # All the bidirectional ports are used for the data_in signal, so they should be inputs
    assert int(dut.uio_oe.value) == 0


    dut._log.info("write 4 bytes to addresses 8, 9, 10, 11")
    dut.ui_in.value = WE | SELECT | 8
    dut.uio_in.value = 0x55
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 9
    dut.uio_in.value = 0x66
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 10
    dut.uio_in.value = 0x77
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 11
    dut.uio_in.value = 0x88
    await ClockCycles(dut.clk, 1)

    dut._log.info("read back the bytes and verify they are correct")
    dut.uio_in.value = 0
    dut.ui_in.value = SELECT | 8
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x55

    dut.ui_in.value = SELECT | 9
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x66

    dut.ui_in.value = SELECT | 10
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x77

    dut.ui_in.value = SELECT | 11
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x88

    dut._log.info("write a byte at address 12")
    dut.ui_in.value = WE | SELECT | 12
    dut.uio_in.value = 0x99
    await ClockCycles(dut.clk, 1)

    dut._log.info("overwrite the byte at address 10")
    dut.ui_in.value = WE | SELECT | 10
    dut.uio_in.value = 0xAA
    await ClockCycles(dut.clk, 1)

    dut._log.info("read back the bytes and verify they are correct")
    dut.uio_in.value = 0
    dut.ui_in.value = SELECT | 12
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x99

    dut.ui_in.value = SELECT | 10
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0xAA

    dut.ui_in.value = SELECT | 8
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x55

    # Reset again
    dut._log.info("Reset")
    dut.rst_n.value = 0
    await ClockCycles(dut.clk, 10)
    dut.rst_n.value = 1

    # Ensure that the memory is cleared
    for i in range(32):
        dut.ui_in.value = SELECT | i
        await ClockCycles(dut.clk, 2)
        assert int(dut.uo_out.value) == 0

    # test data memory, TODO, doesn't seems to work

    SELECT = 1 << 6

    # Reset
    dut._log.info("Reset - Test Data Memory")
    dut.ena.value = 1
    dut.ui_in.value = SELECT | 0
    dut.uio_in.value = 0
    dut.rst_n.value = 0
    await ClockCycles(dut.clk, 10)
    dut.rst_n.value = 1

    # All the bidirectional ports are used for the data_in signal, so they should be inputs
    assert int(dut.uio_oe.value) == 0

    dut._log.info("write 4 bytes to addresses 8, 9, 10, 11")
    dut.ui_in.value = WE | SELECT | 8
    dut.uio_in.value = 0x55
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 9
    dut.uio_in.value = 0x66
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 10
    dut.uio_in.value = 0x77
    await ClockCycles(dut.clk, 1)

    dut.ui_in.value = WE | SELECT | 11
    dut.uio_in.value = 0x88
    await ClockCycles(dut.clk, 1)

    dut._log.info("read back the bytes and verify they are correct")
    dut.uio_in.value = 0
    dut.ui_in.value = SELECT | 8
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x55

    dut.ui_in.value = SELECT | 9
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x66

    dut.ui_in.value = SELECT | 10
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x77

    dut.ui_in.value = SELECT | 11
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x88

    dut._log.info("write a byte at address 12")
    dut.ui_in.value = WE | SELECT | 12
    dut.uio_in.value = 0x99
    await ClockCycles(dut.clk, 1)

    dut._log.info("overwrite the byte at address 10")
    dut.ui_in.value = WE | SELECT | 10
    dut.uio_in.value = 0xAA
    await ClockCycles(dut.clk, 1)

    dut._log.info("read back the bytes and verify they are correct")
    dut.uio_in.value = 0
    dut.ui_in.value = SELECT | 12
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x99

    dut.ui_in.value = SELECT | 10
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0xAA

    dut.ui_in.value = SELECT | 8
    await ClockCycles(dut.clk, 2)
    assert int(dut.uo_out.value) == 0x55

    # Reset again
    dut._log.info("Reset")
    dut.rst_n.value = 0
    await ClockCycles(dut.clk, 10)
    dut.rst_n.value = 1

    # Ensure that the memory is cleared
    for i in range(32):
        dut.ui_in.value = SELECT |  i
        await ClockCycles(dut.clk, 2)
        assert int(dut.uo_out.value) == 0

    dut._log.info("all good!")
