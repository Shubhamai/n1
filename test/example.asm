mov r0 4 // load intermediate value 1 into register 0
mov r1 2 // load intermediate value 2 into register 1
add r3 r1 r0 // add the values in register 0 and register 1 and store the result in register 3
sub r3 r3 r1 // subtract the value in register 1 from the value in register 3 and store the result in register 3
mul r3 r3 r1 // multiply the value in register 3 by the value in register 1 and store the result in register 3
store 0x10 r3 // store the value in register 3 in memory address 0x10
print 0x10 // print the value stored in memory address 0x10
end // end the program

// https://polysoftit.co.uk/irisc-web/

// load, store, move