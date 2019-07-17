
.macro ldr_unaligned addr,value
    // Load the value as a word
    ldr \value,[\addr]
    // Find the sub-word byte offset
    and \addr,#0b11
    // Convert bytes to bits
    lsl \addr,#8
    // Save off only the byte in addr
    lsr \value,\addr
    and \value,#0xFF
.endm
