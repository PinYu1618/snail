# Commands:
#	make build
#   make run
#   make clean
#
# Options:
# 	ARCH = riscv64 | i386

ARCH := riscv64
MODE := release
LOG ?=

target := riscv64gc-unknown-none-elf
build_dir := target/$(target)/$(MODE)
kernel_elf := $(build_dir)/snail-kernel
kernel_bin := $(kernel_elf).bin
fs_img := ../user/target/$(target)/$(MODE)/fs.img
apps := ../user/src/bin/*

objdump := rust-objdump --arch-name=$(ARCH)
objcopy := rust-objcopy --binary-architecture=$(ARCH)

qemu := ~/qemu-5.0.0/riscv64-softmmu/qemu-system-riscv64

qemu_opts = \
	-machine virt \
	-nographic \
	-bios ../boot/$(ARCH)/rustsbi-qemu/target/$(target)/release/rustsbi-qemu.bin \
	-device loader,addr=0x80200000,file=$(kernel_bin) \
	-drive file=$(fs_img),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

ifeq ($(MODE), release)
build_args = --release
endif

build: env $(kernel_bin) fs-img

clean:
	@cargo clean

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(target)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

run: run-inner

run-inner: build
	$(qemu) $(qemu_opts)

$(kernel_bin): kernel
	@$(objcopy) $(kernel_elf) --strip-all -O binary $@

kernel:
	@cargo build $(build_args)

fs-img: $(apps)
	@cd ../user && make build
	@rm -f $(fs_img)
	@cd ../tools && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/

.PHONY: all clean build kernel run run-inner fs-img