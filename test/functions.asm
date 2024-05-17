.entry main

foo
    cmp r2 r3
    jumple +3 // if a<=0
    add r3 r3 1
    ret // return foo(a+1)
    mov r0 r3 // return a   
    ret // return a

main
    mov r3 -10
    call foo
    end

// bl [mem_addr] - branch and link
// bx lr - change program counter to the value of a register ( usually lr )

// any function
// push lr
// do something, call other functions
// pop lr
// bx lr

// foo(a):
//     if a<=0:
//         return foo(a+1)
//     else:
//         return a
// main():
//     foo(-10)
// https://www.youtube.com/watch?v=hKXNr8oAkk8