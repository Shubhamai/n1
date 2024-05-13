.entry main

foo
    cmp r2 r3
    jumpne 0xd // if a<=0
    add r3 r3 1
    ret // return foo(a+1)
    mov r0 r3 // return a   
    ret // return a

main
    mov r3 -10
    call foo
    end