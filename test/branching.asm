mov r3 #14
mov r2 #14
cmp r2 r3
jumpne 0xd // - 13th line
mov r0 #4 // load intermediate value 1 into register 0
mov r1 #2 // load intermediate value 2 into register 1
add r2 r1 r0 // add the values in register 0 and register 1 and store the result in register 3
sub r2 r2 r1 // subtract the value in register 1 from the value in register 3 and store the result in register 3
mul r2 r2 r1 // multiply the value in register 3 by the value in register 1 and store the result in register 3
store 0x64 r2 // store the value in register 3 in memory address 0x64
print 0x64 // print the value stored in memory address 0x64
jump 0xf // - 16th line
mov r2 #42
store 0x64 r2 // store the value in register 3 in memory address 0x64
print 0x64 // print the value stored in memory address 0x64
end // end the program

// https://polysoftit.co.uk/irisc-web/

// load, store, move