mov r1 99 // load intermediate value 1 into register 0
// mov r1 2 // load intermediate value 2 into register 1
// add r3 r1 r0 // add the values in register 0 and register 1 and store the result in register 3
store 0x10 r1 // store the value in register 3 in memory address 0x10
print 0x10 // print the value stored in memory address 0x10
end // end the program

// https://polysoftit.co.uk/irisc-web/

// load, store, move