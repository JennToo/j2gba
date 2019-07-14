.global main
.global __IRQ_HANDLER

.section .text
.thumb

main:
    b main

__IRQ_HANDLER:
    bx lr
