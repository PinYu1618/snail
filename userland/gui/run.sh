#! /usr/bin/env bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLO='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

QEMU='/home/pycc/qemu-5.0.0/riscv64-linux-user/qemu-riscv64'

HELLO_APP_ELF='target/debug/examples/hello'
HELLO_APP_BIN=${HELLO_APP_ELF}.bin

${QEMU} ${HELLO_APP_ELF}