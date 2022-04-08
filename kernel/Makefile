# Commands:
#	make build
#   make run
#   make clean
#
# Options:
# 	ARCH = riscv64 | i386

ARCH ?= riscv64
MODE ?= debug
LOG ?=
GRAPHIC ?= off
SMP ?= 1

target := riscv64gc-unknown-none-elf
build_dir := target/$(target)/$(MODE)
kernel := $(build_dir)/snail
kernel_img := $(build_dir)/kernel.img
fs_img := ../user/target/$(target)/$(MODE)/fs.img

###### qemu ######
ifeq ($(ARCH), riscv64)
qemu := ~/qemu-5.0.0/riscv64-softmmu/qemu-system-riscv64
endif

###### qemu options ######
qemu_opts := \
	-smp cores=$(SMP)

ifeq ($(ARCH), riscv64)
qemu_opts += \
	-machine virt \
	-cpu rv64,x-h=true \
	-bios ../boot/$(ARCH)/rustsbi-qemu/target/$(target)/release/rustsbi-qemu.bin  \
	-device loader,addr=0x80200000,file=$(kernel_img) 
#	-drive file=$(fs_img),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0
endif

ifeq ($(GRAPHIC), off)
qemu_opts += -nographic
endif

###### build args ######
#build_args := \
	--target targets/$(target).json
#	-Z build-std=core,alloc

ifeq ($(MODE), release)
build_args = --release
endif

sysroot := $(shell rustc --print sysroot)
objcopy := rust-objcopy

.PHONY: all clean build kernel run justrun test justtest

all: kernel

clean:
	@cargo clean

run: build justrun
test: build justtest

justrun:
	$(qemu) $(qemu_opts) $(kernel_img)

justtest:
# TODO

build: $(kernel_img)

###### bootloader and kernel image ######

bootloader: $(kernel)
ifeq ($(ARCH), x86_64)
	@cd ../boot && make build
endif

$(kernel_img): kernel bootloader
ifeq ($(ARCH), riscv64))
	@$(objcopy) $(kernel) --strip-all -O binary $@
endif

kernel:
	@echo Building $(ARCH) kernel
	@cargo build $(build_args)