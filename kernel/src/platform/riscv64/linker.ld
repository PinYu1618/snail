OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x80200000;

SECTIONS
{
    . = BASE_ADDR;

    .text : {
        *(.text .text.*)
    }
}