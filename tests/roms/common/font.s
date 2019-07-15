.global font_gfx
.global font_pal

.section .text
.thumb
.align 4

font_gfx:
.incbin "font.bin"

font_pal:
.incbin "font.bin.pal"
