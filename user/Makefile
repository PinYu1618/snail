TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin

target_dir := target/$(TARGET)/$(MODE)
apps := $(wildcard $(APP_DIR)/*.rs)
elfs := $(patsubst $(APP_DIR)/%.rs, $(target_dir)/%, $(apps))
BINS := $(patsubst $(APP_DIR)/%.rs, $(target_dir)/%.bin, $(apps))

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

elf: $(apps)
	@cargo build --release

binary: elf
	$(foreach elf, $(elfs), $(objcopy) $(elf) --strip-all -O binary $(patsubst $(target_dir)/%, $(target_dir)/%.bin, $(elf));)

build: binary

clean:
	@cargo clean

.PHONY: elf binary build clean