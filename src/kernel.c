#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include "colors.h"

uint16_t *vga_buffer = (uint16_t *)0xB8000;


uint16_t get_color_vga(uint16_t color) {
   return (color << 8);
}

uint16_t get_letter_with_color(uint16_t color, uint16_t letter) {
  return (get_color_vga(color) | letter);
}

void kernel_main(void)
{
  *vga_buffer = get_letter_with_color(RED, 'A');
  while (true) {}
}