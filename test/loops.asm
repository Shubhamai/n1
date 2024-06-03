.entry main

main:
    mov r0 #1 // init loop value
    mov r1 #1 // increment value
    mov r2 #10 // loop end value
    mov r3 #0 // sum value from inside loop
    jump 0x6
    add r3 r3 r2
    add r0 r0 r1
    cmp r0 r2
    jumple 0x4
    store 0x64 r3 // store the value in register 3 in memory address 0x64
    print 0x64 // print the value stored in memory address 0x64
    end // end the program

// for (r0 = 1; r0 < r2; r0 += r1) {
//     r3 += r2;
// }