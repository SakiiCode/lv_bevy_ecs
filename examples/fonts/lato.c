/*******************************************************************************
 * Size: 14 px
 * Bpp: 1
 * Opts: --bpp 1 --size 14 -o lato.c --format lvgl --font Lato-Regular.ttf -r 0x0-0x7F
 ******************************************************************************/

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif

#ifndef LATO
#define LATO 1
#endif

#if LATO

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0000 "\u0000" */
    0x0,

    /* U+000D "\r" */
    0x0,

    /* U+0020 " " */
    0x0,

    /* U+0021 "!" */
    0xfc, 0x40,

    /* U+0022 "\"" */
    0xb6, 0x80,

    /* U+0023 "#" */
    0x12, 0x16, 0x14, 0x7f, 0x24, 0x24, 0xfe, 0x28,
    0x48, 0x48,

    /* U+0024 "$" */
    0x11, 0xed, 0x64, 0x91, 0xc3, 0xc9, 0x26, 0xbf,
    0x88, 0x20,

    /* U+0025 "%" */
    0x60, 0xa4, 0x49, 0x22, 0x48, 0x64, 0x2, 0x61,
    0x24, 0x49, 0x22, 0x50, 0x60,

    /* U+0026 "&" */
    0x38, 0x22, 0x10, 0x8, 0x2, 0x2, 0x8a, 0x25,
    0xc, 0xc7, 0x3c, 0xc0,

    /* U+0027 "'" */
    0xe0,

    /* U+0028 "(" */
    0x9, 0x49, 0x24, 0x91, 0x22,

    /* U+0029 ")" */
    0x9, 0x32, 0x49, 0x25, 0x28,

    /* U+002A "*" */
    0x23, 0x9c, 0x40,

    /* U+002B "+" */
    0x10, 0x20, 0x47, 0xf1, 0x2, 0x4, 0x0,

    /* U+002C "," */
    0xd8,

    /* U+002D "-" */
    0xe0,

    /* U+002E "." */
    0x80,

    /* U+002F "/" */
    0x8, 0x44, 0x22, 0x10, 0x88, 0x44, 0x0,

    /* U+0030 "0" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x44,
    0x70,

    /* U+0031 "1" */
    0x11, 0xcd, 0x4, 0x10, 0x41, 0x4, 0x11, 0xf0,

    /* U+0032 "2" */
    0x7b, 0x18, 0x41, 0xc, 0x21, 0x8, 0x43, 0xf0,

    /* U+0033 "3" */
    0x3c, 0xc5, 0x8, 0x31, 0xc0, 0xc0, 0xa1, 0x66,
    0x78,

    /* U+0034 "4" */
    0x4, 0xc, 0x14, 0x24, 0x64, 0x44, 0xff, 0x4,
    0x4, 0x4,

    /* U+0035 "5" */
    0x7d, 0x4, 0x10, 0xf8, 0x30, 0x41, 0x8d, 0xe0,

    /* U+0036 "6" */
    0xc, 0x30, 0xc3, 0x7, 0xd8, 0xe0, 0xc1, 0xc6,
    0xf0,

    /* U+0037 "7" */
    0xfe, 0xc, 0x10, 0x60, 0x83, 0x4, 0x18, 0x20,
    0xc0,

    /* U+0038 "8" */
    0x79, 0xa, 0x14, 0x27, 0x98, 0xa0, 0xc1, 0xc6,
    0xf8,

    /* U+0039 "9" */
    0x7b, 0x38, 0x61, 0xcd, 0xe0, 0x84, 0x21, 0x80,

    /* U+003A ":" */
    0xc0, 0xc,

    /* U+003B ";" */
    0xc0, 0xd, 0x80,

    /* U+003C "<" */
    0x4, 0x67, 0x30, 0x30, 0x20,

    /* U+003D "=" */
    0xfc, 0xf, 0xc0,

    /* U+003E ">" */
    0x3, 0x3, 0x3, 0x33, 0x0,

    /* U+003F "?" */
    0x74, 0x42, 0x11, 0x10, 0x80, 0x1, 0x0,

    /* U+0040 "@" */
    0x1e, 0x8, 0x64, 0xe, 0x39, 0x9a, 0x64, 0x99,
    0x2e, 0x7e, 0x40, 0x8, 0x21, 0xf0,

    /* U+0041 "A" */
    0xc, 0xe, 0x5, 0x2, 0x43, 0x21, 0x19, 0xfc,
    0x82, 0x40, 0xe0, 0x40,

    /* U+0042 "B" */
    0xfd, 0xe, 0xc, 0x18, 0x7f, 0xa1, 0xc1, 0x87,
    0xf8,

    /* U+0043 "C" */
    0x1e, 0x61, 0x40, 0x80, 0x80, 0x80, 0x80, 0x40,
    0x61, 0x3e,

    /* U+0044 "D" */
    0xfc, 0x86, 0x82, 0x81, 0x81, 0x81, 0x81, 0x82,
    0x86, 0xfc,

    /* U+0045 "E" */
    0xfe, 0x8, 0x20, 0x83, 0xe8, 0x20, 0x83, 0xf0,

    /* U+0046 "F" */
    0xfe, 0x8, 0x20, 0x83, 0xe8, 0x20, 0x82, 0x0,

    /* U+0047 "G" */
    0x1f, 0x30, 0xd0, 0x10, 0x8, 0x4, 0x1e, 0x2,
    0x81, 0x60, 0x8f, 0x80,

    /* U+0048 "H" */
    0x81, 0x81, 0x81, 0x81, 0x81, 0xff, 0x81, 0x81,
    0x81, 0x81,

    /* U+0049 "I" */
    0xff, 0xc0,

    /* U+004A "J" */
    0x11, 0x11, 0x11, 0x11, 0x3e,

    /* U+004B "K" */
    0x86, 0x84, 0x88, 0x98, 0xb0, 0xe0, 0x90, 0x88,
    0x84, 0x82,

    /* U+004C "L" */
    0x84, 0x21, 0x8, 0x42, 0x10, 0x87, 0xc0,

    /* U+004D "M" */
    0xc0, 0xf0, 0x3e, 0x1e, 0x85, 0xb3, 0x64, 0x99,
    0xe6, 0x31, 0x80, 0x60, 0x10,

    /* U+004E "N" */
    0x81, 0xc1, 0xe1, 0xb1, 0x99, 0x99, 0x8d, 0x87,
    0x83, 0x81,

    /* U+004F "O" */
    0x1e, 0x18, 0x64, 0xa, 0x1, 0x80, 0x60, 0x18,
    0x5, 0x2, 0x61, 0x87, 0x80,

    /* U+0050 "P" */
    0xfd, 0xe, 0xc, 0x18, 0x7f, 0x20, 0x40, 0x81,
    0x0,

    /* U+0051 "Q" */
    0x1e, 0x18, 0x64, 0xa, 0x1, 0x80, 0x60, 0x18,
    0x5, 0x3, 0x61, 0x87, 0xc0, 0x18, 0x3,

    /* U+0052 "R" */
    0xf9, 0x1a, 0x14, 0x28, 0xde, 0x24, 0x44, 0x8d,
    0xc,

    /* U+0053 "S" */
    0x7a, 0x18, 0x30, 0x70, 0x70, 0x41, 0x8f, 0xe0,

    /* U+0054 "T" */
    0xff, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10,

    /* U+0055 "U" */
    0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81,
    0x42, 0x3c,

    /* U+0056 "V" */
    0xc0, 0xa0, 0xd0, 0x4c, 0x22, 0x31, 0x90, 0x58,
    0x28, 0xc, 0x6, 0x0,

    /* U+0057 "W" */
    0xc3, 0x5, 0xc, 0x34, 0x38, 0x99, 0xa2, 0x24,
    0x98, 0x93, 0x43, 0xc5, 0xe, 0x1c, 0x18, 0x70,
    0x60, 0x80,

    /* U+0058 "X" */
    0x41, 0x31, 0x88, 0x82, 0x81, 0xc0, 0xe0, 0xd8,
    0x44, 0x63, 0x60, 0xc0,

    /* U+0059 "Y" */
    0x41, 0x31, 0x88, 0x86, 0xc1, 0x40, 0x40, 0x20,
    0x10, 0x8, 0x4, 0x0,

    /* U+005A "Z" */
    0xfe, 0xc, 0x30, 0x41, 0x86, 0x8, 0x30, 0xc1,
    0xfc,

    /* U+005B "[" */
    0xf2, 0x49, 0x24, 0x92, 0x70,

    /* U+005C "\\" */
    0x84, 0x10, 0x82, 0x10, 0x42, 0x10, 0x40,

    /* U+005D "]" */
    0xe4, 0x92, 0x49, 0x24, 0xf0,

    /* U+005E "^" */
    0x0, 0xc7, 0x92, 0xc4,

    /* U+005F "_" */
    0xfc,

    /* U+0060 "`" */
    0x44,

    /* U+0061 "a" */
    0x39, 0x10, 0x4f, 0x45, 0x17, 0xc0,

    /* U+0062 "b" */
    0x82, 0x8, 0x2e, 0xce, 0x18, 0x61, 0x8f, 0xe0,

    /* U+0063 "c" */
    0x7e, 0x21, 0x8, 0x61, 0xe0,

    /* U+0064 "d" */
    0x4, 0x10, 0x5f, 0xc6, 0x18, 0x61, 0xcd, 0xd0,

    /* U+0065 "e" */
    0x39, 0x8a, 0x17, 0xf8, 0x8, 0xf, 0x0,

    /* U+0066 "f" */
    0x34, 0x4f, 0x44, 0x44, 0x44,

    /* U+0067 "g" */
    0x7f, 0xa, 0x13, 0xc8, 0xf, 0xa1, 0x42, 0x78,

    /* U+0068 "h" */
    0x82, 0x8, 0x2e, 0xc6, 0x18, 0x61, 0x86, 0x10,

    /* U+0069 "i" */
    0xc2, 0xaa, 0xa0,

    /* U+006A "j" */
    0x30, 0x2, 0x22, 0x22, 0x22, 0x2e,

    /* U+006B "k" */
    0x82, 0x8, 0x22, 0x92, 0x8e, 0x2c, 0x9a, 0x20,

    /* U+006C "l" */
    0xff, 0xc0,

    /* U+006D "m" */
    0xb7, 0x64, 0x62, 0x31, 0x18, 0x8c, 0x46, 0x22,

    /* U+006E "n" */
    0xbb, 0x18, 0x61, 0x86, 0x18, 0x40,

    /* U+006F "o" */
    0x38, 0x8a, 0xc, 0x18, 0x28, 0x8e, 0x0,

    /* U+0070 "p" */
    0xbb, 0x38, 0x61, 0x86, 0x3f, 0xa0, 0x80,

    /* U+0071 "q" */
    0x7f, 0x18, 0x61, 0x87, 0x37, 0x41, 0x4,

    /* U+0072 "r" */
    0xbc, 0x88, 0x88, 0x80,

    /* U+0073 "s" */
    0x74, 0x70, 0xe0, 0xcf, 0xc0,

    /* U+0074 "t" */
    0x44, 0xf4, 0x44, 0x44, 0x70,

    /* U+0075 "u" */
    0x86, 0x18, 0x61, 0x86, 0x37, 0x40,

    /* U+0076 "v" */
    0xc2, 0x89, 0x11, 0x62, 0x87, 0x4, 0x0,

    /* U+0077 "w" */
    0xc4, 0x49, 0x89, 0x29, 0x25, 0x43, 0x28, 0x63,
    0xc, 0x60,

    /* U+0078 "x" */
    0x44, 0xd8, 0xa0, 0x82, 0x8d, 0x91, 0x80,

    /* U+0079 "y" */
    0xc2, 0x89, 0x91, 0x42, 0x83, 0x4, 0x18, 0x20,

    /* U+007A "z" */
    0x7c, 0x21, 0x84, 0x21, 0xf, 0xc0,

    /* U+007B "{" */
    0x69, 0x25, 0x12, 0x49, 0x10,

    /* U+007C "|" */
    0xff, 0xf8,

    /* U+007D "}" */
    0x89, 0x24, 0x52, 0x49, 0x40,

    /* U+007E "~" */
    0xe6, 0x78, 0x0
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 0, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 1, .adv_w = 0, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 2, .adv_w = 43, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 3, .adv_w = 77, .box_w = 1, .box_h = 10, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 5, .adv_w = 89, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 7, .adv_w = 130, .box_w = 8, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 17, .adv_w = 130, .box_w = 6, .box_h = 13, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 27, .adv_w = 176, .box_w = 10, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 40, .adv_w = 157, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 52, .adv_w = 52, .box_w = 1, .box_h = 3, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 53, .adv_w = 67, .box_w = 3, .box_h = 13, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 58, .adv_w = 67, .box_w = 3, .box_h = 13, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 63, .adv_w = 90, .box_w = 5, .box_h = 4, .ofs_x = 0, .ofs_y = 6},
    {.bitmap_index = 66, .adv_w = 130, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 1},
    {.bitmap_index = 73, .adv_w = 47, .box_w = 2, .box_h = 3, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 74, .adv_w = 78, .box_w = 3, .box_h = 1, .ofs_x = 1, .ofs_y = 4},
    {.bitmap_index = 75, .adv_w = 47, .box_w = 1, .box_h = 1, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 76, .adv_w = 84, .box_w = 5, .box_h = 11, .ofs_x = 0, .ofs_y = -1},
    {.bitmap_index = 83, .adv_w = 130, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 92, .adv_w = 130, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 100, .adv_w = 130, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 108, .adv_w = 130, .box_w = 7, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 117, .adv_w = 130, .box_w = 8, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 127, .adv_w = 130, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 135, .adv_w = 130, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 144, .adv_w = 130, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 153, .adv_w = 130, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 162, .adv_w = 130, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 170, .adv_w = 56, .box_w = 2, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 172, .adv_w = 56, .box_w = 2, .box_h = 9, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 175, .adv_w = 130, .box_w = 6, .box_h = 6, .ofs_x = 1, .ofs_y = 2},
    {.bitmap_index = 180, .adv_w = 130, .box_w = 6, .box_h = 3, .ofs_x = 1, .ofs_y = 3},
    {.bitmap_index = 183, .adv_w = 130, .box_w = 6, .box_h = 6, .ofs_x = 2, .ofs_y = 2},
    {.bitmap_index = 188, .adv_w = 89, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 195, .adv_w = 184, .box_w = 10, .box_h = 11, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 209, .adv_w = 152, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 221, .adv_w = 145, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 230, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 240, .adv_w = 169, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 250, .adv_w = 130, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 258, .adv_w = 127, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 266, .adv_w = 164, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 278, .adv_w = 169, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 288, .adv_w = 69, .box_w = 1, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 290, .adv_w = 99, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 295, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 305, .adv_w = 115, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 312, .adv_w = 206, .box_w = 10, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 325, .adv_w = 169, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 335, .adv_w = 179, .box_w = 10, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 348, .adv_w = 137, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 357, .adv_w = 179, .box_w = 10, .box_h = 12, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 372, .adv_w = 144, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 381, .adv_w = 119, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 389, .adv_w = 132, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 399, .adv_w = 164, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 409, .adv_w = 152, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 421, .adv_w = 228, .box_w = 14, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 439, .adv_w = 144, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 451, .adv_w = 141, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 463, .adv_w = 140, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 472, .adv_w = 67, .box_w = 3, .box_h = 12, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 477, .adv_w = 84, .box_w = 5, .box_h = 11, .ofs_x = 0, .ofs_y = -1},
    {.bitmap_index = 484, .adv_w = 67, .box_w = 3, .box_h = 12, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 489, .adv_w = 130, .box_w = 6, .box_h = 5, .ofs_x = 1, .ofs_y = 5},
    {.bitmap_index = 493, .adv_w = 88, .box_w = 6, .box_h = 1, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 494, .adv_w = 69, .box_w = 3, .box_h = 2, .ofs_x = 0, .ofs_y = 8},
    {.bitmap_index = 495, .adv_w = 114, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 501, .adv_w = 125, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 509, .adv_w = 105, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 514, .adv_w = 125, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 522, .adv_w = 117, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 529, .adv_w = 75, .box_w = 4, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 534, .adv_w = 114, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 542, .adv_w = 125, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 550, .adv_w = 57, .box_w = 2, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 553, .adv_w = 57, .box_w = 4, .box_h = 12, .ofs_x = -1, .ofs_y = -2},
    {.bitmap_index = 559, .adv_w = 117, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 567, .adv_w = 57, .box_w = 1, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 569, .adv_w = 184, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 577, .adv_w = 125, .box_w = 6, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 583, .adv_w = 125, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 590, .adv_w = 124, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 597, .adv_w = 125, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 604, .adv_w = 90, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 608, .adv_w = 97, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 613, .adv_w = 84, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 618, .adv_w = 125, .box_w = 6, .box_h = 7, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 624, .adv_w = 115, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 631, .adv_w = 172, .box_w = 11, .box_h = 7, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 641, .adv_w = 113, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 648, .adv_w = 115, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 656, .adv_w = 103, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 662, .adv_w = 67, .box_w = 3, .box_h = 12, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 667, .adv_w = 67, .box_w = 1, .box_h = 13, .ofs_x = 2, .ofs_y = -2},
    {.bitmap_index = 669, .adv_w = 67, .box_w = 3, .box_h = 12, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 674, .adv_w = 130, .box_w = 6, .box_h = 3, .ofs_x = 1, .ofs_y = 2}
};

/*---------------------
 *  CHARACTER MAPPING
 *--------------------*/

static const uint16_t unicode_list_0[] = {
    0x0, 0xd
};

/*Collect the unicode lists and glyph_id offsets*/
static const lv_font_fmt_txt_cmap_t cmaps[] =
{
    {
        .range_start = 0, .range_length = 14, .glyph_id_start = 1,
        .unicode_list = unicode_list_0, .glyph_id_ofs_list = NULL, .list_length = 2, .type = LV_FONT_FMT_TXT_CMAP_SPARSE_TINY
    },
    {
        .range_start = 32, .range_length = 95, .glyph_id_start = 3,
        .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0, .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY
    }
};

/*-----------------
 *    KERNING
 *----------------*/


/*Map glyph_ids to kern left classes*/
static const uint8_t kern_left_class_mapping[] =
{
    0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 1, 2, 0, 1, 0, 3,
    4, 3, 5, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 6, 7, 0, 8, 6,
    0, 9, 0, 0, 0, 10, 11, 12,
    0, 0, 6, 13, 6, 14, 0, 15,
    10, 5, 16, 11, 17, 18, 2, 7,
    0, 0, 0, 0, 19, 20, 0, 0,
    20, 21, 0, 19, 0, 0, 22, 0,
    19, 19, 20, 20, 0, 23, 0, 0,
    0, 24, 25, 22, 24, 0, 2, 0,
    0, 0
};

/*Map glyph_ids to kern right classes*/
static const uint8_t kern_right_class_mapping[] =
{
    0, 0, 0, 0, 0, 1, 0, 0,
    0, 2, 1, 0, 3, 1, 0, 4,
    5, 4, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 6, 6, 0,
    0, 0, 7, 8, 2, 0, 8, 0,
    0, 0, 8, 0, 0, 9, 0, 0,
    0, 0, 8, 0, 8, 0, 0, 10,
    11, 12, 13, 14, 15, 16, 0, 12,
    3, 0, 0, 0, 17, 0, 18, 18,
    18, 19, 20, 0, 0, 0, 0, 0,
    6, 6, 18, 6, 18, 6, 21, 22,
    6, 23, 24, 25, 23, 26, 0, 0,
    3, 0
};

/*Kern values between classes*/
static const int8_t kern_class_values[] =
{
    0, -20, 0, -26, -20, 0, 0, -5,
    0, 0, 0, 5, 5, 0, 3, 0,
    -7, -10, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, -4, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -4, 0, 0, 0, 0,
    0, 0, 0, 0, -26, 0, 0, 0,
    -15, 0, 0, -6, 0, -20, 0, -20,
    -14, 0, -17, 0, 0, 0, 0, 0,
    0, 0, -15, -7, 0, 0, -20, -6,
    0, -15, 0, 0, 0, 0, 0, -20,
    0, -13, -4, -7, -18, -5, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    5, -15, 0, -22, -13, -10, 5, -6,
    -17, 0, 0, 0, 0, 0, 0, 0,
    -13, -13, -3, -15, -12, -5, -5, 0,
    -6, -9, -5, -5, -4, -6, 0, 0,
    0, 0, 0, -11, 0, -6, 0, -3,
    -9, -8, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, -20, 0, 0, 0,
    -6, 0, -6, -5, 6, -15, -6, -15,
    -9, 0, -18, 0, 0, 0, 0, 0,
    0, 0, -9, 0, 0, 0, 0, 0,
    0, 0, -17, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, -15, 0, -20, 0, -7, 3, 0,
    -22, 0, 0, 0, 0, 0, 0, 0,
    0, -8, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -6, 0, -6, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    -7, 0, 0, -3, 0, 0, 0, 0,
    0, 0, 0, 0, 0, -4, -6, 0,
    0, -9, -7, -6, 0, 0, -32, 0,
    0, 6, -22, 0, -6, -9, 0, -19,
    0, -20, -17, 0, -24, 0, 0, -4,
    0, 0, 0, 0, -12, -9, 0, 0,
    0, -15, 0, -28, 0, 0, 0, 0,
    -20, 0, 0, 0, 0, 0, 0, 0,
    -6, -3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, -5, 0, -6, -5, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, -15, 0, -20,
    -20, -18, 0, -11, -22, 0, 0, 0,
    0, 0, 0, 0, -28, -24, 0, -21,
    -18, 0, -20, -16, -16, -13, 5, -11,
    0, -14, -4, 0, 4, 0, -11, 0,
    0, 0, 0, 0, 0, 0, -10, -4,
    0, -11, -5, 0, 0, 0, 0, 0,
    3, -18, 0, -17, -18, -14, 4, -9,
    -22, 0, 0, 0, 0, 0, 0, 0,
    -14, -18, 0, -19, -14, 0, -11, -10,
    -15, 0, 0, 0, 0, 0, -8, 0,
    4, -6, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -4, 0, 0, -3, 0,
    -4, 0, 0, 0, -8, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, -4, -2, 0, 0, -10, 0,
    -4, 0, 0, 0, 0, 0, 0, 0,
    0, -13, -4, 0, 0, 0, 0, 0,
    0, 0, 0, 0, -3, 0, -7, 0,
    8, 0, 0, -15, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -7, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, -15,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, -4, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, -9,
    0, -15, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, -3,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -7, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
};


/*Collect the kern class' data in one place*/
static const lv_font_fmt_txt_kern_classes_t kern_classes =
{
    .class_pair_values   = kern_class_values,
    .left_class_mapping  = kern_left_class_mapping,
    .right_class_mapping = kern_right_class_mapping,
    .left_class_cnt      = 25,
    .right_class_cnt     = 26,
};

/*--------------------
 *  ALL CUSTOM DATA
 *--------------------*/

#if LVGL_VERSION_MAJOR == 8
/*Store all the custom data of the font*/
static  lv_font_fmt_txt_glyph_cache_t cache;
#endif

#if LVGL_VERSION_MAJOR >= 8
static const lv_font_fmt_txt_dsc_t font_dsc = {
#else
static lv_font_fmt_txt_dsc_t font_dsc = {
#endif
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = &kern_classes,
    .kern_scale = 16,
    .cmap_num = 2,
    .bpp = 1,
    .kern_classes = 1,
    .bitmap_format = 0,
#if LVGL_VERSION_MAJOR == 8
    .cache = &cache
#endif
};



/*-----------------
 *  PUBLIC FONT
 *----------------*/

/*Initialize a public general font descriptor*/
#if LVGL_VERSION_MAJOR >= 8
const lv_font_t lato = {
#else
lv_font_t lato = {
#endif
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 13,          /*The maximum line height required by the font*/
    .base_line = 2,             /*Baseline measured from the bottom of the line*/
#if !(LVGL_VERSION_MAJOR == 6 && LVGL_VERSION_MINOR == 0)
    .subpx = LV_FONT_SUBPX_NONE,
#endif
#if LV_VERSION_CHECK(7, 4, 0) || LVGL_VERSION_MAJOR >= 8
    .underline_position = -1,
    .underline_thickness = 1,
#endif
    .dsc = &font_dsc,          /*The custom font data. Will be accessed by `get_glyph_bitmap/dsc` */
#if LV_VERSION_CHECK(8, 2, 0) || LVGL_VERSION_MAJOR >= 9
    .fallback = NULL,
#endif
    .user_data = NULL,
};



#endif /*#if LATO*/

