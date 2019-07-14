AS = arm-none-eabi-as
LD = arm-none-eabi-ld
OBJCOPY = arm-none-eabi-objcopy

ASM_SRCS := $(shell find tests/roms -type f -name '*.s')
ASM_OBJS := $(ASM_SRCS:tests/roms/%.s=target/test-roms/%.o)

ROM_SRCS := $(wildcard tests/roms/*.s)
ROM_ELVES := $(ROM_SRCS:tests/roms/%.s=target/test-roms/%.elf)

COMMON_SRCS := $(wildcard tests/roms/common/*.s)
COMMON_OBJS := $(COMMON_SRCS:tests/roms/%.s=target/test-roms/%.o)

LDFLAGS := -T linker.ld
ASFLAGS := -march=armv4t

all: $(ROM_ELVES)

# TODO: The dependencies could be reduced, but this is easy for now
$(ROM_ELVES): $(ASM_OBJS)
	$(LD) $(LDFLAGS) $(COMMON_OBJS) $(@:%.elf=%.o) -o $@

$(ASM_OBJS): target/test-roms/%.o: tests/roms/%.s
	mkdir -p $(@D)
	$(AS) $(ASFLAGS) $^ -o $@

clean:
	rm -rf target/test-roms
