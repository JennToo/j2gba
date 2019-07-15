AS = arm-none-eabi-as
LD = arm-none-eabi-ld
OBJCOPY = arm-none-eabi-objcopy

ASM_SRCS := $(shell find tests/roms -type f -name '*.s')
ASM_OBJS := $(ASM_SRCS:tests/roms/%.s=target/test-roms/%.o)

ASM_INCLUDES := $(shell find tests/roms -type f -name '*.inc')

ROM_SRCS := $(wildcard tests/roms/*.s)
ROM_ELVES := $(ROM_SRCS:tests/roms/%.s=target/test-roms/%.elf)
ROMS := $(ROM_ELVES:%.elf=%.gba)

COMMON_SRCS := $(wildcard tests/roms/common/*.s)
COMMON_OBJS := $(COMMON_SRCS:tests/roms/%.s=target/test-roms/%.o)

LDFLAGS := -T linker.ld
ASFLAGS := -march=armv4t -I target/test-roms/common -I tests/roms/common

FONT_SRC := tests/roms/common/font.png
FONT_BIN := target/test-roms/common/font.bin

all: $(ROMS)

$(ROMS): target/test-roms/%.gba: target/test-roms/%.elf
	$(OBJCOPY) -O binary $< $@
	/opt/devkitpro/tools/bin/gbafix $@

# TODO: The dependencies could be reduced, but this is easy for now
$(ROM_ELVES): $(ASM_OBJS)
	$(LD) $(LDFLAGS) $(COMMON_OBJS) $(@:%.elf=%.o) -o $@

$(ASM_OBJS): target/test-roms/%.o: tests/roms/%.s $(FONT_BIN) $(ASM_INCLUDES)
	mkdir -p $(@D)
	$(AS) $(ASFLAGS) $< -o $@

$(FONT_BIN): $(FONT_SRC)
	mkdir -p $(@D)
	j2-gba-tool gfx-convert bg256c1p $^ $@ $@.pal

clean:
	rm -rf target/test-roms
