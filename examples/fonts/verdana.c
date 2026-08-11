/*******************************************************************************
 * Size: 14 px
 * Bpp: 1
 * Opts: --bpp 1 --size 14 -o verdana.c --format lvgl --font Verdana.ttf -r 0x0-0x7F --autohint-off
 ******************************************************************************/

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif

#ifndef VERDANA
#define VERDANA 1
#endif

#if VERDANA

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
    0x0,

    /* U+0021 "!" */
    0xfe, 0xc0,

    /* U+0022 "\"" */
    0x99, 0x99,

    /* U+0023 "#" */
    0x9, 0x4, 0x84, 0x8f, 0xf1, 0x21, 0x23, 0xfc,
    0x48, 0x48, 0x24, 0x0,

    /* U+0024 "$" */
    0x10, 0x21, 0xf4, 0x99, 0x12, 0x1f, 0x9, 0x13,
    0x25, 0xf0, 0x81, 0x0,

    /* U+0025 "%" */
    0x70, 0x44, 0x44, 0x22, 0x41, 0x12, 0x8, 0xa7,
    0x39, 0x44, 0x12, 0x21, 0x11, 0x8, 0x88, 0x83,
    0x80,

    /* U+0026 "&" */
    0x78, 0x21, 0x8, 0x42, 0x20, 0x71, 0x22, 0x48,
    0x52, 0x8, 0x45, 0xe, 0x20,

    /* U+0027 "'" */
    0xf0,

    /* U+0028 "(" */
    0x12, 0x44, 0x88, 0x88, 0x88, 0x44, 0x21,

    /* U+0029 ")" */
    0x84, 0x22, 0x11, 0x11, 0x11, 0x22, 0x48,

    /* U+002A "*" */
    0x11, 0x25, 0x51, 0xc5, 0x52, 0x44, 0x0,

    /* U+002B "+" */
    0x8, 0x4, 0x2, 0x1, 0xf, 0xf8, 0x40, 0x20,
    0x10, 0x8, 0x0,

    /* U+002C "," */
    0x49, 0x40,

    /* U+002D "-" */
    0xf8,

    /* U+002E "." */
    0xc0,

    /* U+002F "/" */
    0x4, 0x20, 0x82, 0x10, 0x42, 0x8, 0x21, 0x4,
    0x20, 0x80,

    /* U+0030 "0" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x44,
    0x70,

    /* U+0031 "1" */
    0x27, 0x8, 0x42, 0x10, 0x84, 0x27, 0xc0,

    /* U+0032 "2" */
    0x7d, 0x4, 0x8, 0x10, 0x43, 0x8, 0x20, 0x81,
    0xfc,

    /* U+0033 "3" */
    0x7d, 0x4, 0x8, 0x23, 0x80, 0x80, 0x81, 0x84,
    0xf0,

    /* U+0034 "4" */
    0x4, 0xc, 0x14, 0x24, 0x44, 0x84, 0xff, 0x4,
    0x4, 0x4,

    /* U+0035 "5" */
    0xff, 0x2, 0x4, 0xf, 0x80, 0x80, 0x81, 0x84,
    0xf0,

    /* U+0036 "6" */
    0x1c, 0xc1, 0x4, 0xf, 0x90, 0xa0, 0xc1, 0x44,
    0x70,

    /* U+0037 "7" */
    0xfe, 0x4, 0x10, 0x20, 0x81, 0x4, 0x10, 0x20,
    0x80,

    /* U+0038 "8" */
    0x7d, 0x6, 0xc, 0x17, 0xd0, 0x60, 0xc1, 0x82,
    0xf8,

    /* U+0039 "9" */
    0x38, 0x8a, 0xc, 0x14, 0x27, 0xc0, 0x82, 0xc,
    0xe0,

    /* U+003A ":" */
    0xc3,

    /* U+003B ";" */
    0x48, 0x0, 0x12, 0x50,

    /* U+003C "<" */
    0x3, 0xc, 0x30, 0xc0, 0xc0, 0x30, 0xc, 0x3,

    /* U+003D "=" */
    0xff, 0x0, 0x0, 0xff,

    /* U+003E ">" */
    0xc0, 0x30, 0xc, 0x3, 0x3, 0xc, 0x30, 0xc0,

    /* U+003F "?" */
    0x7a, 0x10, 0x41, 0x18, 0x82, 0x0, 0x20, 0x80,

    /* U+0040 "@" */
    0xf, 0x83, 0x4, 0x40, 0x24, 0xf9, 0x90, 0x99,
    0x9, 0x90, 0x99, 0x9, 0x4f, 0xe4, 0x0, 0x30,
    0x0, 0xf8,

    /* U+0041 "A" */
    0xc, 0x4, 0x81, 0x20, 0x84, 0x21, 0x8, 0x47,
    0xf9, 0x2, 0x80, 0x60, 0x10,

    /* U+0042 "B" */
    0xfc, 0x82, 0x82, 0x82, 0xfc, 0x82, 0x81, 0x81,
    0x82, 0xfc,

    /* U+0043 "C" */
    0x1e, 0x61, 0x40, 0x80, 0x80, 0x80, 0x80, 0x40,
    0x61, 0x1e,

    /* U+0044 "D" */
    0xfc, 0x41, 0xa0, 0x50, 0x18, 0xc, 0x6, 0x3,
    0x2, 0x83, 0x7e, 0x0,

    /* U+0045 "E" */
    0xff, 0x2, 0x4, 0xf, 0xf0, 0x20, 0x40, 0x81,
    0xfc,

    /* U+0046 "F" */
    0xff, 0x2, 0x4, 0xf, 0xd0, 0x20, 0x40, 0x81,
    0x0,

    /* U+0047 "G" */
    0x1e, 0x30, 0xd0, 0x30, 0x8, 0x4, 0x3e, 0x2,
    0x81, 0x60, 0x8f, 0x80,

    /* U+0048 "H" */
    0x81, 0x81, 0x81, 0x81, 0xff, 0x81, 0x81, 0x81,
    0x81, 0x81,

    /* U+0049 "I" */
    0xe9, 0x24, 0x92, 0x5c,

    /* U+004A "J" */
    0x38, 0x42, 0x10, 0x84, 0x21, 0xf, 0x80,

    /* U+004B "K" */
    0x82, 0x84, 0x88, 0x90, 0xa0, 0xd0, 0x88, 0x84,
    0x82, 0x81,

    /* U+004C "L" */
    0x81, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x81,
    0xfc,

    /* U+004D "M" */
    0xc1, 0xe0, 0xe8, 0xb4, 0x59, 0x4c, 0xa6, 0x23,
    0x11, 0x80, 0xc0, 0x40,

    /* U+004E "N" */
    0xc1, 0xc1, 0xa1, 0x91, 0x91, 0x89, 0x89, 0x85,
    0x83, 0x83,

    /* U+004F "O" */
    0x1c, 0x31, 0x90, 0x50, 0x18, 0xc, 0x6, 0x2,
    0x82, 0x63, 0xe, 0x0,

    /* U+0050 "P" */
    0xf9, 0xa, 0xc, 0x18, 0x5f, 0x20, 0x40, 0x81,
    0x0,

    /* U+0051 "Q" */
    0x1c, 0x31, 0x90, 0x50, 0x18, 0xc, 0x6, 0x2,
    0x82, 0x63, 0xe, 0x1, 0x0, 0x80, 0x38,

    /* U+0052 "R" */
    0xf8, 0x84, 0x84, 0x84, 0x88, 0xf0, 0x88, 0x84,
    0x82, 0x81,

    /* U+0053 "S" */
    0x3e, 0x41, 0x80, 0x80, 0x70, 0x1e, 0x1, 0x81,
    0x42, 0x3c,

    /* U+0054 "T" */
    0xff, 0x84, 0x2, 0x1, 0x0, 0x80, 0x40, 0x20,
    0x10, 0x8, 0x4, 0x0,

    /* U+0055 "U" */
    0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81,
    0x42, 0x3c,

    /* U+0056 "V" */
    0x80, 0x60, 0x14, 0x9, 0x2, 0x21, 0x8, 0x42,
    0x10, 0x48, 0x12, 0x3, 0x0,

    /* U+0057 "W" */
    0x82, 0xc, 0x10, 0x51, 0x44, 0x8a, 0x24, 0x51,
    0x14, 0x50, 0xa2, 0x85, 0x14, 0x10, 0x40, 0x82,
    0x0,

    /* U+0058 "X" */
    0x81, 0x42, 0x24, 0x24, 0x18, 0x18, 0x24, 0x24,
    0x42, 0x81,

    /* U+0059 "Y" */
    0x80, 0xa0, 0x88, 0x84, 0x41, 0x40, 0x40, 0x20,
    0x10, 0x8, 0x4, 0x0,

    /* U+005A "Z" */
    0xff, 0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40,
    0x80, 0xff,

    /* U+005B "[" */
    0xf8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x8f,

    /* U+005C "\\" */
    0x82, 0x4, 0x10, 0x20, 0x82, 0x4, 0x10, 0x20,
    0x82, 0x4,

    /* U+005D "]" */
    0xf1, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1f,

    /* U+005E "^" */
    0x8, 0xa, 0x8, 0x88, 0x28, 0x8,

    /* U+005F "_" */
    0xff, 0x80,

    /* U+0060 "`" */
    0x44,

    /* U+0061 "a" */
    0x39, 0x10, 0x5f, 0x86, 0x18, 0xdd,

    /* U+0062 "b" */
    0x81, 0x2, 0x5, 0xcc, 0x50, 0x60, 0xc1, 0x83,
    0xb, 0xe0,

    /* U+0063 "c" */
    0x39, 0x18, 0x20, 0x82, 0x4, 0x4e,

    /* U+0064 "d" */
    0x2, 0x4, 0x9, 0xf4, 0x30, 0x60, 0xc1, 0x82,
    0x8c, 0xe8,

    /* U+0065 "e" */
    0x31, 0x28, 0x7f, 0x82, 0x4, 0x4e,

    /* U+0066 "f" */
    0x3a, 0x11, 0xf4, 0x21, 0x8, 0x42, 0x10,

    /* U+0067 "g" */
    0x3e, 0x86, 0xc, 0x18, 0x30, 0x51, 0x9d, 0x2,
    0x88, 0xe0,

    /* U+0068 "h" */
    0x81, 0x2, 0x5, 0xec, 0x30, 0x60, 0xc1, 0x83,
    0x6, 0x8,

    /* U+0069 "i" */
    0xbf, 0xc0,

    /* U+006A "j" */
    0x10, 0x71, 0x11, 0x11, 0x11, 0x11, 0xe0,

    /* U+006B "k" */
    0x81, 0x2, 0x4, 0x28, 0x92, 0x28, 0x70, 0x91,
    0x12, 0x10,

    /* U+006C "l" */
    0xff, 0xe0,

    /* U+006D "m" */
    0xb9, 0xd8, 0xc6, 0x10, 0xc2, 0x18, 0x43, 0x8,
    0x61, 0xc, 0x21,

    /* U+006E "n" */
    0xbd, 0x86, 0xc, 0x18, 0x30, 0x60, 0xc1,

    /* U+006F "o" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x51, 0x1c,

    /* U+0070 "p" */
    0xb9, 0x8a, 0xc, 0x18, 0x30, 0x61, 0x7c, 0x81,
    0x2, 0x0,

    /* U+0071 "q" */
    0x3e, 0x86, 0xc, 0x18, 0x30, 0x51, 0xbd, 0x2,
    0x4, 0x8,

    /* U+0072 "r" */
    0xbe, 0x21, 0x8, 0x42, 0x10,

    /* U+0073 "s" */
    0x7a, 0x18, 0x18, 0x18, 0x18, 0x5e,

    /* U+0074 "t" */
    0x42, 0x3e, 0x84, 0x21, 0x8, 0x41, 0xc0,

    /* U+0075 "u" */
    0x83, 0x6, 0xc, 0x18, 0x30, 0x61, 0xbd,

    /* U+0076 "v" */
    0x86, 0x14, 0x92, 0x48, 0xc3, 0xc,

    /* U+0077 "w" */
    0x88, 0xc4, 0x55, 0x4a, 0xa5, 0x52, 0xa8, 0x88,
    0x44,

    /* U+0078 "x" */
    0x82, 0x88, 0xa0, 0x81, 0x5, 0x11, 0x41,

    /* U+0079 "y" */
    0x86, 0x14, 0x92, 0x48, 0xc3, 0xc, 0x20, 0x84,
    0x0,

    /* U+007A "z" */
    0xfc, 0x10, 0x84, 0x21, 0x8, 0x3f,

    /* U+007B "{" */
    0xe, 0x20, 0x40, 0x81, 0x4, 0x30, 0x10, 0x10,
    0x20, 0x40, 0x81, 0x1, 0xc0,

    /* U+007C "|" */
    0xff, 0xfc,

    /* U+007D "}" */
    0xe0, 0x20, 0x40, 0x81, 0x1, 0x1, 0x84, 0x10,
    0x20, 0x40, 0x81, 0x1c, 0x0,

    /* U+007E "~" */
    0x70, 0xc4, 0x62, 0x30, 0xe0
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 79, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 1, .adv_w = 88, .box_w = 1, .box_h = 10, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 3, .adv_w = 103, .box_w = 4, .box_h = 4, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 5, .adv_w = 183, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 17, .adv_w = 142, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 29, .adv_w = 241, .box_w = 13, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 46, .adv_w = 163, .box_w = 10, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 59, .adv_w = 60, .box_w = 1, .box_h = 4, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 60, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 67, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 74, .adv_w = 142, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 4},
    {.bitmap_index = 81, .adv_w = 183, .box_w = 9, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 92, .adv_w = 81, .box_w = 3, .box_h = 4, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 94, .adv_w = 102, .box_w = 5, .box_h = 1, .ofs_x = 1, .ofs_y = 4},
    {.bitmap_index = 95, .adv_w = 81, .box_w = 1, .box_h = 2, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 96, .adv_w = 102, .box_w = 6, .box_h = 13, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 106, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 115, .adv_w = 142, .box_w = 5, .box_h = 10, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 122, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 131, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 140, .adv_w = 142, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 150, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 159, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 168, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 177, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 186, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 195, .adv_w = 102, .box_w = 1, .box_h = 8, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 196, .adv_w = 102, .box_w = 3, .box_h = 10, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 200, .adv_w = 183, .box_w = 8, .box_h = 8, .ofs_x = 1, .ofs_y = 1},
    {.bitmap_index = 208, .adv_w = 183, .box_w = 8, .box_h = 4, .ofs_x = 2, .ofs_y = 2},
    {.bitmap_index = 212, .adv_w = 183, .box_w = 8, .box_h = 8, .ofs_x = 2, .ofs_y = 1},
    {.bitmap_index = 220, .adv_w = 122, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 228, .adv_w = 224, .box_w = 12, .box_h = 12, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 246, .adv_w = 153, .box_w = 10, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 259, .adv_w = 154, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 269, .adv_w = 156, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 279, .adv_w = 173, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 291, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 300, .adv_w = 129, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 309, .adv_w = 174, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 321, .adv_w = 168, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 331, .adv_w = 94, .box_w = 3, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 335, .adv_w = 102, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 342, .adv_w = 155, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 352, .adv_w = 125, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 361, .adv_w = 189, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 373, .adv_w = 168, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 383, .adv_w = 176, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 395, .adv_w = 135, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 404, .adv_w = 176, .box_w = 9, .box_h = 13, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 419, .adv_w = 156, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 429, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 439, .adv_w = 138, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 451, .adv_w = 164, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 461, .adv_w = 153, .box_w = 10, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 474, .adv_w = 221, .box_w = 13, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 491, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 501, .adv_w = 138, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 513, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 523, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 2, .ofs_y = -3},
    {.bitmap_index = 530, .adv_w = 102, .box_w = 6, .box_h = 13, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 540, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 547, .adv_w = 183, .box_w = 9, .box_h = 5, .ofs_x = 1, .ofs_y = 5},
    {.bitmap_index = 553, .adv_w = 142, .box_w = 9, .box_h = 1, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 555, .adv_w = 142, .box_w = 3, .box_h = 2, .ofs_x = 2, .ofs_y = 9},
    {.bitmap_index = 556, .adv_w = 135, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 562, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 572, .adv_w = 117, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 578, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 588, .adv_w = 133, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 594, .adv_w = 79, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 601, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 611, .adv_w = 142, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 621, .adv_w = 61, .box_w = 1, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 623, .adv_w = 77, .box_w = 4, .box_h = 13, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 630, .adv_w = 133, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 640, .adv_w = 61, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 642, .adv_w = 218, .box_w = 11, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 653, .adv_w = 142, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 660, .adv_w = 136, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 667, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 677, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 687, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 692, .adv_w = 117, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 698, .adv_w = 88, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 705, .adv_w = 142, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 712, .adv_w = 133, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 718, .adv_w = 183, .box_w = 9, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 727, .adv_w = 133, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 734, .adv_w = 133, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 743, .adv_w = 118, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 749, .adv_w = 142, .box_w = 7, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 762, .adv_w = 102, .box_w = 1, .box_h = 14, .ofs_x = 3, .ofs_y = -3},
    {.bitmap_index = 764, .adv_w = 142, .box_w = 7, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 777, .adv_w = 183, .box_w = 9, .box_h = 4, .ofs_x = 1, .ofs_y = 2}
};

/*---------------------
 *  CHARACTER MAPPING
 *--------------------*/



/*Collect the unicode lists and glyph_id offsets*/
static const lv_font_fmt_txt_cmap_t cmaps[] =
{
    {
        .range_start = 32, .range_length = 95, .glyph_id_start = 1,
        .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0, .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY
    }
};

/*-----------------
 *    KERNING
 *----------------*/


/*Pair left and right glyphs for kerning*/
static const uint8_t kern_pair_glyph_ids[] =
{
    8, 34,
    14, 34,
    14, 42,
    14, 43,
    14, 52,
    14, 53,
    14, 55,
    14, 56,
    14, 57,
    14, 58,
    14, 59,
    14, 66,
    14, 87,
    14, 88,
    14, 89,
    14, 90,
    14, 91,
    15, 13,
    15, 14,
    34, 14,
    34, 52,
    34, 53,
    34, 54,
    34, 55,
    34, 56,
    34, 58,
    34, 85,
    34, 86,
    34, 87,
    34, 88,
    34, 90,
    35, 13,
    35, 14,
    35, 15,
    35, 53,
    36, 14,
    37, 13,
    37, 15,
    37, 53,
    37, 56,
    37, 57,
    37, 58,
    37, 59,
    39, 13,
    39, 15,
    39, 27,
    39, 28,
    39, 32,
    39, 34,
    39, 53,
    39, 66,
    39, 70,
    39, 80,
    42, 14,
    43, 13,
    43, 15,
    43, 34,
    44, 14,
    44, 48,
    44, 66,
    44, 70,
    44, 80,
    44, 86,
    44, 87,
    44, 88,
    44, 90,
    45, 8,
    45, 14,
    45, 36,
    45, 40,
    45, 43,
    45, 48,
    45, 53,
    45, 55,
    45, 56,
    45, 58,
    45, 87,
    45, 90,
    48, 13,
    48, 15,
    48, 53,
    48, 57,
    48, 58,
    48, 59,
    49, 13,
    49, 15,
    49, 34,
    49, 58,
    49, 66,
    49, 70,
    49, 80,
    50, 13,
    50, 15,
    51, 14,
    51, 53,
    51, 58,
    51, 66,
    51, 70,
    51, 80,
    51, 86,
    51, 90,
    52, 13,
    52, 15,
    52, 34,
    52, 52,
    52, 87,
    52, 88,
    52, 90,
    53, 13,
    53, 14,
    53, 15,
    53, 27,
    53, 28,
    53, 32,
    53, 34,
    53, 36,
    53, 40,
    53, 48,
    53, 52,
    53, 53,
    53, 66,
    53, 68,
    53, 70,
    53, 72,
    53, 80,
    53, 83,
    53, 84,
    53, 86,
    53, 87,
    53, 88,
    53, 90,
    53, 91,
    54, 13,
    54, 15,
    54, 34,
    55, 13,
    55, 14,
    55, 15,
    55, 27,
    55, 28,
    55, 34,
    55, 66,
    55, 70,
    55, 80,
    55, 86,
    55, 90,
    56, 13,
    56, 14,
    56, 15,
    56, 27,
    56, 28,
    56, 34,
    56, 66,
    56, 70,
    56, 80,
    56, 83,
    56, 86,
    56, 90,
    57, 14,
    57, 36,
    57, 40,
    57, 48,
    57, 66,
    57, 70,
    57, 80,
    57, 86,
    57, 90,
    58, 13,
    58, 14,
    58, 15,
    58, 27,
    58, 28,
    58, 34,
    58, 48,
    58, 66,
    58, 69,
    58, 70,
    58, 72,
    58, 78,
    58, 79,
    58, 80,
    58, 81,
    58, 82,
    58, 83,
    58, 84,
    58, 86,
    58, 87,
    59, 14,
    59, 36,
    59, 40,
    59, 48,
    59, 59,
    59, 66,
    59, 70,
    59, 80,
    59, 88,
    59, 90,
    66, 87,
    66, 88,
    66, 90,
    67, 13,
    67, 15,
    67, 90,
    68, 14,
    68, 53,
    70, 53,
    71, 3,
    71, 8,
    71, 10,
    71, 11,
    71, 13,
    71, 14,
    71, 15,
    71, 32,
    71, 61,
    71, 62,
    71, 90,
    71, 94,
    73, 87,
    73, 88,
    73, 90,
    76, 14,
    76, 70,
    76, 80,
    78, 87,
    78, 88,
    78, 90,
    79, 87,
    79, 88,
    79, 90,
    80, 13,
    80, 15,
    80, 87,
    80, 89,
    80, 90,
    81, 13,
    81, 15,
    81, 90,
    83, 13,
    83, 14,
    83, 15,
    83, 66,
    85, 14,
    85, 90,
    87, 13,
    87, 14,
    87, 15,
    87, 66,
    87, 70,
    87, 80,
    88, 13,
    88, 14,
    88, 15,
    88, 66,
    89, 14,
    89, 68,
    89, 69,
    89, 70,
    89, 72,
    89, 80,
    90, 13,
    90, 14,
    90, 15,
    90, 66,
    90, 68,
    90, 69,
    90, 70,
    90, 72,
    90, 80,
    90, 82,
    91, 14,
    91, 68,
    91, 69,
    91, 70,
    91, 72,
    91, 80,
    91, 82
};

/* Kerning between the respective left and right glyphs
 * 4.4 format which needs to scaled with `kern_scale`*/
static const int8_t kern_pair_values[] =
{
    -11, -5, -3, -11, -2, -16, -5, -5,
    -9, -15, -3, -2, -4, -2, -5, -4,
    -4, -14, -17, -5, -1, -13, -1, -7,
    -5, -9, -2, -1, -5, -3, -5, -2,
    1, -2, -7, -5, -5, -5, -5, -2,
    -1, -2, -2, -33, -33, -7, -7, 7,
    -11, 3, -11, -5, -5, -3, -2, -2,
    -1, -12, -2, -7, -8, -8, -5, -9,
    -8, -9, -13, -17, -2, -2, 11, -2,
    -19, -12, -11, -17, -12, -12, -3, -3,
    -5, -1, -2, -2, -33, -33, -5, 2,
    -5, -5, -5, -3, -3, -11, -7, -2,
    -4, -5, -5, -2, -6, -2, -2, -2,
    -2, -3, -2, -3, -32, -16, -32, -22,
    -22, 7, -13, -4, -4, -5, -1, -8,
    -26, -24, -24, -23, -24, -22, -20, -22,
    -22, -22, -22, -19, -2, -2, -1, -32,
    -5, -32, -9, -9, -7, -11, -11, -11,
    -7, -7, -32, -5, -24, -9, -9, -5,
    -11, -11, -11, -7, -7, -7, -9, -1,
    -1, -1, -5, -7, -7, -3, -9, -32,
    -15, -32, -22, -22, -9, -2, -15, -13,
    -14, -14, -11, -11, -14, -11, -14, -11,
    -12, -12, -11, -7, -2, -2, -2, -1,
    -5, -7, -7, -4, -7, -2, -1, -2,
    -3, -2, -1, -2, -7, -15, 7, 7,
    11, 5, -14, -5, -13, 12, 11, 11,
    -1, 11, -2, -1, -2, -11, -2, -2,
    -2, -1, -2, -2, -1, -2, -3, -2,
    -2, -2, -2, -3, -2, -1, -32, -2,
    -32, -4, -4, -1, -20, -4, -20, -4,
    -2, -2, -8, -2, -8, -2, -5, -2,
    -1, -3, -1, -3, -21, -4, -21, -4,
    -2, -1, -2, -1, -2, -1, -2, -1,
    -1, -1, -1, -1, -1
};

/*Collect the kern pair's data in one place*/
static const lv_font_fmt_txt_kern_pair_t kern_pairs =
{
    .glyph_ids = kern_pair_glyph_ids,
    .values = kern_pair_values,
    .pair_cnt = 277,
    .glyph_ids_size = 0
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
    .kern_dsc = &kern_pairs,
    .kern_scale = 16,
    .cmap_num = 1,
    .bpp = 1,
    .kern_classes = 0,
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
const lv_font_t verdana = {
#else
lv_font_t verdana = {
#endif
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 14,          /*The maximum line height required by the font*/
    .base_line = 3,             /*Baseline measured from the bottom of the line*/
#if !(LVGL_VERSION_MAJOR == 6 && LVGL_VERSION_MINOR == 0)
    .subpx = LV_FONT_SUBPX_NONE,
#endif
#if LV_VERSION_CHECK(7, 4, 0) || LVGL_VERSION_MAJOR >= 8
    .underline_position = -2,
    .underline_thickness = 1,
#endif
    .dsc = &font_dsc,          /*The custom font data. Will be accessed by `get_glyph_bitmap/dsc` */
#if LV_VERSION_CHECK(8, 2, 0) || LVGL_VERSION_MAJOR >= 9
    .fallback = NULL,
#endif
    .user_data = NULL,
};



#endif /*#if VERDANA*/

