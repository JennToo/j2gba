.global simple_memcpy

.section .text
.thumb
.align 4

// This memcpy isn't supposed to be fast, it's supposed to require as few CPU
// features as possible
//
// r0 - destination
// r1 - source
// r2 - byte_count
// r3 - clobbered
//
// byte_count must be divisible by 4
// source and destination must be word aligned
simple_memcpy:
    cmp r2,#0
    beq .Ldone
    ldr r3,[r1]
    str r3,[r0]
    add r0,#4
    add r1,#4
    sub r2,#4
    b simple_memcpy
.Ldone:
    bx lr
