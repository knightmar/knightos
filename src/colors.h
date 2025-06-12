//
// Created by knightmar on 30/05/25.
//

#ifndef COLORS_H
#define COLORS_H


#define BLACK         0x0 // 0000
#define BLUE          0x1 // 0001
#define GREEN         0x2 // 0010
#define CYAN          0x3 // 0011
#define RED           0x4 // 0100
#define MAGENTA       0x5 // 0101
#define BROWN         0x6 // 0110
#define LIGHT_GREY    0x7 // 0111
#define DARK_GREY     0x8 // 1000
#define LIGHT_BLUE    0x9 // 1001
#define LIGHT_GREEN   0xA // 1010
#define LIGHT_CYAN    0xB // 1011
#define LIGHT_RED     0xC // 1100
#define LIGHT_MAGENTA 0xD // 1101
#define LIGHT_BROWN   0xE // 1110 (often called Yellow)
#define WHITE         0xF // 1111


uint16_t get_color_vga(uint16_t color);

#endif //COLORS_H
