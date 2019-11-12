
.macro ldr_unaligned addr,value,temporary
    // Load the value as a word
    ldr \value,[\addr]
    // Find the sub-word byte offset
    ldr \temporary,=#0b11
    and \addr,\temporary
    // Convert bytes to bits
    ldr \temporary,=#8
    lsl \addr,\temporary
    // Save off only the byte in addr
    lsr \value,\addr
    ldr \temporary,=#0xFF
    and \value,\temporary
.endm
