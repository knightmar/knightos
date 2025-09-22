ENTRY(_start)

PHDRS
{
  text   PT_LOAD FLAGS(0x5); /* R X */
  rodata PT_LOAD FLAGS(0x4); /* R   */
  data   PT_LOAD FLAGS(0x6); /* R W */
}

SECTIONS
{
    . = 2M;

    .text : ALIGN(0x1000)
    {
        *(.multiboot)
        *(.text*)
    } :text

    .rodata : ALIGN(0x1000)
    {
        *(.rodata*)
    } :rodata

    .data : ALIGN(0x1000)
    {
        *(.data*)
    } :data

    .bss : ALIGN(0x1000)
    {
        *(COMMON)
        *(.bss*)
        *(COMMON*)
    } :data
}