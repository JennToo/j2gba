.global main
.global __IRQ_HANDLER

.section .text
.thumb
.align 4

main:
    ldr r0,=0x6000000
    ldr r1,=font_gfx
    ldr r2,=8192
    bl simple_memcpy

    ldr r0,=0x5000000
    ldr r1,=font_pal
    ldr r2,=512
    bl simple_memcpy

    ldr r0,=0x4000000
    ldr r1,=0x0
    strh r1,[r0]

    ldr r0,=0x4000008
    ldr r1,=0b0000100010000000
    strh r1,[r0]

.Ldone:
    b .Ldone

irq_handler:
    bx lr

.align 4
__IRQ_HANDLER:
.int irq_handler + 1
