#
#

TARGET = aarch64-unknown-none

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld


XRUSTC_CMD   = cargo xrustc --target=$(TARGET) --release
CARGO_OUTPUT = target/$(TARGET)/release/kernel8

OBJCOPY        = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary

CONTAINER_UTILS   = andrerichter/raspi3-utils

DOCKER_CMD        = docker run -it --rm
DOCKER_ARG_CURDIR = -v $(shell pwd):/work -w /work
DOCKER_ARG_TTY    = --privileged -v /dev:/dev

DOCKER_EXEC_QEMU     = qemu-system-aarch64 -M raspi3 -kernel kernel8.img
DOCKER_EXEC_RASPBOOT = raspbootcom /dev/ttyUSB0

.PHONY: all qemu raspboot clippy clean objdump nm

all: clean kernel8.img

$(CARGO_OUTPUT): $(SOURCES)
	$(XRUSTC_CMD)

kernel8.img: $(CARGO_OUTPUT)
	cp $< .
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< kernel8.img

qemu: all
	$(DOCKER_CMD) $(DOCKER_ARG_CURDIR) $(CONTAINER_UTILS) \
	$(DOCKER_EXEC_QEMU) -serial stdio

raspboot: all
	$(DOCKER_CMD) $(DOCKER_ARG_CURDIR) $(DOCKER_ARG_TTY) \
	$(CONTAINER_UTILS) $(DOCKER_EXEC_RASPBOOT) kernel8.img

clippy:
	cargo xclippy --target=$(TARGET)

clean:
	cargo clean

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex kernel8

nm:
	cargo nm --target $(TARGET) -- kernel8 | sort