.entry main

main:
    mov r3 #14
    mov r2 #6

    add r1 r2 r3

    zstore 0x64 r1 
    print 0x64
 

    end