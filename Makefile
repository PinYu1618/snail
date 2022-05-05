# Commands:
#	make build
#   make run
#	make test
#   make clean
#
# Options:
# 	ARCH = riscv64 | x86_64
#	MODE = debug | release

ARCH ?= riscv64
MODE ?= debug
LOG ?=

###### target triple ######
ifeq ($(ARCH), riscv64)
target_triple = riscv64gc-unknown-none-elf
else ifeq ($(ARCH), x86_64)
target_triple = x86_64-unknown-none
endif

build_dir := target/$(target_triple)/$(MODE)
kernel_elf := $(build_dir)/snail
kernel_bin := $(kernel_elf).bin
apps := user/src/bin/*

###### user target triple ######
ifeq ($(ARCH), riscv64)
app_target_triple = riscv64gc-unknown-none-elf
endif
app_build_dir = user/target/$(app_target_triple)/release
app_img = $(app_build_dir)/fs.img

###### binutils ######
objdump := rust-objdump --arch-name=$(ARCH)
objcopy := rust-objcopy --binary-architecture=$(ARCH)

###### qemu ######
ifeq ($(ARCH), riscv64)
qemu = ~/qemu-5.0.0/riscv64-softmmu/qemu-system-riscv64
else
qemu = qemu-system-$(ARCH)
endif

###### qemu options ######
ifeq ($(ARCH), riscv64)
qemu_opts = \
	-machine virt \
	-bios $(build_dir)/boot-riscv64.bin \
	-device loader,addr=0x80200000,file=$(kernel_bin) \
	-drive file=$(app_img),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-nographic
else ifeq ($(ARCH), x86_64)
qemu_opts = \
	-machine virt
endif

build: env bootloader $(kernel_bin) fs-img

clean:
	@cargo clean

env:
	(rustup target list | grep "$(target_triple) (installed)") || rustup target add $(target_triple)
	@cargo install cargo-binutils --vers =0.3.3
	@rustup component add rust-src
	@rustup component add llvm-tools-preview

run: run-inner

run-inner: build
	@$(qemu) $(qemu_opts)

$(kernel_bin): kernel
	@$(objcopy) $(kernel_elf) --strip-all -O binary $@

kernel:
	@cd kernel && cargo build $(build_args)

fs-img: $(apps)
	@cd user && cargo build --release
	@rm -f $(app_img)
	@cd tools && cargo run --release -- -s ../user/src/bin/ -t ../$(app_build_dir)/

bootloader:
	@cd boot && cargo build $(build_args)
	@$(objcopy) $(build_dir)/boot-$(ARCH) --strip-all -O binary $(build_dir)/boot-$(ARCH).bin

.PHONY: all clean build kernel run run-inner fs-img bootloader
