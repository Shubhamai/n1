.entry main

foo:
    // cmp r2 r3
    // jumple +3 // if a<=0
    // add r3 r3 r1
    // call foo // return foo(a+1)
    store 0x64 r3 // store the value in register 3 in memory address 0x64
    print 0x64 // print the value stored in memory address 0x64 
    ret // return a

main:
    mov r3 #10
    mov r2 #0
    mov r1 #1
    call foo
    end

// call [func_name -> mem_addr] - jump to mem_addr, push current pc to stack
// ret - pop pc from stack, jump to popped pc

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