/*******************************************************************************
 * Size: 14 px
 * Bpp: 1
 * Opts: --bpp 1 --size 14 -o verdana.c --format lvgl --font Verdana.ttf -r 0x0-0x7F
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
    0xfe, 0x40,

    /* U+0022 "\"" */
    0xdd, 0x99,

    /* U+0023 "#" */
    0x9, 0x5, 0x1f, 0xe2, 0x41, 0x20, 0xa3, 0xfc,
    0x48, 0x24, 0x14, 0x0,

    /* U+0024 "$" */
    0x10, 0x21, 0xfc, 0x99, 0x1a, 0x1f, 0xb, 0x13,
    0x27, 0xf0, 0x81, 0x2, 0x0,

    /* U+0025 "%" */
    0x70, 0x84, 0x44, 0x22, 0x41, 0x12, 0x8, 0xa7,
    0x39, 0x44, 0x12, 0x20, 0x91, 0x8, 0x88, 0x43,
    0x80,

    /* U+0026 "&" */
    0x38, 0x11, 0x4, 0x41, 0xb0, 0x31, 0x12, 0x48,
    0x52, 0x8, 0xc7, 0x1f, 0x20,

    /* U+0027 "'" */
    0xfa,

    /* U+0028 "(" */
    0x36, 0x4c, 0x88, 0x88, 0x88, 0xc4, 0x62,

    /* U+0029 ")" */
    0x46, 0x22, 0x11, 0x11, 0x11, 0x22, 0x6c,

    /* U+002A "*" */
    0x11, 0xac, 0xe1, 0xcd, 0x62, 0x0,

    /* U+002B "+" */
    0x8, 0x4, 0x2, 0x1f, 0xf0, 0x80, 0x40, 0x20,
    0x10,

    /* U+002C "," */
    0x6d, 0x68,

    /* U+002D "-" */
    0xf0,

    /* U+002E "." */
    0xc0,

    /* U+002F "/" */
    0xc, 0x20, 0x86, 0x10, 0x42, 0x8, 0x61, 0x4,
    0x30, 0x80,

    /* U+0030 "0" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x44,
    0x70,

    /* U+0031 "1" */
    0x23, 0x82, 0x8, 0x20, 0x82, 0x8, 0x23, 0xf0,

    /* U+0032 "2" */
    0xf9, 0x8, 0x10, 0x20, 0xc1, 0x4, 0x30, 0xc1,
    0xfc,

    /* U+0033 "3" */
    0x7c, 0x84, 0x8, 0x31, 0x80, 0xc0, 0x81, 0x86,
    0xf8,

    /* U+0034 "4" */
    0x4, 0xc, 0x14, 0x24, 0x44, 0x84, 0xff, 0x4,
    0x4, 0x4,

    /* U+0035 "5" */
    0x7e, 0x81, 0x3, 0xe0, 0x60, 0x40, 0x81, 0xc5,
    0xf0,

    /* U+0036 "6" */
    0x1c, 0xc2, 0x7, 0xe8, 0x70, 0x60, 0xc1, 0x44,
    0x70,

    /* U+0037 "7" */
    0xfe, 0x4, 0x10, 0x60, 0x83, 0x4, 0x18, 0x20,
    0xc0,

    /* U+0038 "8" */
    0x7d, 0x8e, 0xe, 0x17, 0xd9, 0xa0, 0xc1, 0xc6,
    0xf8,

    /* U+0039 "9" */
    0x39, 0x8a, 0xc, 0x1c, 0x2f, 0xc0, 0x82, 0xc,
    0xe0,

    /* U+003A ":" */
    0xc3,

    /* U+003B ";" */
    0x48, 0x0, 0x12, 0x4a, 0x0,

    /* U+003C "<" */
    0x0, 0x3, 0x1c, 0x70, 0xc0, 0x70, 0xc, 0x3,

    /* U+003D "=" */
    0xff, 0x0, 0x0, 0xff,

    /* U+003E ">" */
    0x0, 0xc0, 0x38, 0xe, 0x3, 0xe, 0x30, 0xc0,

    /* U+003F "?" */
    0xfa, 0x10, 0x41, 0x8, 0xc2, 0x8, 0x0, 0x80,

    /* U+0040 "@" */
    0xf, 0x83, 0x4, 0x4f, 0x2d, 0x91, 0x91, 0x19,
    0x11, 0x91, 0x19, 0x11, 0x4f, 0xe2, 0x0, 0x1f,
    0x0,

    /* U+0041 "A" */
    0xc, 0xe, 0x5, 0x82, 0x43, 0x21, 0x19, 0xfc,
    0x82, 0x41, 0xe0, 0x40,

    /* U+0042 "B" */
    0xfd, 0x6, 0xc, 0x3f, 0xd0, 0xe0, 0xc1, 0x87,
    0xf8,

    /* U+0043 "C" */
    0x1f, 0x30, 0xd0, 0x10, 0x8, 0x4, 0x2, 0x1,
    0x80, 0x61, 0x8f, 0x80,

    /* U+0044 "D" */
    0xf8, 0x86, 0x82, 0x81, 0x81, 0x81, 0x81, 0x82,
    0x86, 0xf8,

    /* U+0045 "E" */
    0xfe, 0x8, 0x20, 0xfe, 0x8, 0x20, 0x83, 0xf0,

    /* U+0046 "F" */
    0xfe, 0x8, 0x20, 0xfa, 0x8, 0x20, 0x82, 0x0,

    /* U+0047 "G" */
    0x1f, 0x30, 0xd0, 0x10, 0x8, 0x4, 0x3e, 0x2,
    0x81, 0x60, 0x8f, 0x80,

    /* U+0048 "H" */
    0x81, 0x81, 0x81, 0x81, 0xff, 0x81, 0x81, 0x81,
    0x81, 0x81,

    /* U+0049 "I" */
    0xf2, 0x22, 0x22, 0x22, 0x2f,

    /* U+004A "J" */
    0x38, 0x42, 0x10, 0x84, 0x21, 0xf, 0x80,

    /* U+004B "K" */
    0x86, 0x84, 0x88, 0x90, 0xa0, 0xf0, 0x98, 0x8c,
    0x84, 0x82,

    /* U+004C "L" */
    0x82, 0x8, 0x20, 0x82, 0x8, 0x20, 0x83, 0xf0,

    /* U+004D "M" */
    0xc1, 0xe0, 0xe8, 0xb4, 0x59, 0x4c, 0xe6, 0x23,
    0x1, 0x80, 0xc0, 0x40,

    /* U+004E "N" */
    0xc1, 0xe1, 0xb1, 0x91, 0x99, 0x89, 0x8d, 0x87,
    0x83, 0x83,

    /* U+004F "O" */
    0x3e, 0x31, 0xb0, 0x70, 0x18, 0xc, 0x6, 0x3,
    0x83, 0x63, 0x1f, 0x0,

    /* U+0050 "P" */
    0xfa, 0x38, 0x61, 0x8f, 0xe8, 0x20, 0x82, 0x0,

    /* U+0051 "Q" */
    0x3e, 0x31, 0xb0, 0x70, 0x18, 0xc, 0x6, 0x2,
    0x83, 0x63, 0xf, 0x0, 0x80, 0x40, 0x18,

    /* U+0052 "R" */
    0xf8, 0x8c, 0x84, 0x84, 0x8c, 0xf8, 0x98, 0x8c,
    0x86, 0x82,

    /* U+0053 "S" */
    0x3e, 0xc3, 0x80, 0x80, 0x78, 0x1e, 0x1, 0x1,
    0xc3, 0x7c,

    /* U+0054 "T" */
    0xff, 0x84, 0x2, 0x1, 0x0, 0x80, 0x40, 0x20,
    0x10, 0x8, 0x4, 0x0,

    /* U+0055 "U" */
    0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81,
    0x42, 0x3c,

    /* U+0056 "V" */
    0xc0, 0xa0, 0xd8, 0x4c, 0x22, 0x31, 0x90, 0x48,
    0x2c, 0x1c, 0x6, 0x0,

    /* U+0057 "W" */
    0x86, 0x18, 0x63, 0xc6, 0x34, 0xb2, 0x49, 0x24,
    0x96, 0x69, 0x43, 0x1c, 0x30, 0xc3, 0xc,

    /* U+0058 "X" */
    0x41, 0x90, 0x8c, 0x82, 0xc0, 0xc0, 0x60, 0x58,
    0x64, 0x21, 0x20, 0xc0,

    /* U+0059 "Y" */
    0x41, 0x31, 0x88, 0x82, 0x81, 0xc0, 0x40, 0x20,
    0x10, 0x8, 0x4, 0x0,

    /* U+005A "Z" */
    0xff, 0x3, 0x6, 0xc, 0x8, 0x10, 0x30, 0x60,
    0xc0, 0xff,

    /* U+005B "[" */
    0xf2, 0x49, 0x24, 0x92, 0x49, 0xc0,

    /* U+005C "\\" */
    0x40, 0x81, 0x81, 0x2, 0x2, 0x4, 0xc, 0x8,
    0x10, 0x30, 0x20, 0x40,

    /* U+005D "]" */
    0xe4, 0x92, 0x49, 0x24, 0x93, 0xc0,

    /* U+005E "^" */
    0x8, 0x6, 0x5, 0x84, 0x46, 0x12, 0xc,

    /* U+005F "_" */
    0xff, 0x80,

    /* U+0060 "`" */
    0xc8, 0x80,

    /* U+0061 "a" */
    0x78, 0x10, 0x5f, 0xc6, 0x18, 0xdd,

    /* U+0062 "b" */
    0x81, 0x2, 0x5, 0xec, 0x50, 0x60, 0xc1, 0x83,
    0xb, 0xe0,

    /* U+0063 "c" */
    0x3d, 0x18, 0x20, 0x82, 0x4, 0x4f,

    /* U+0064 "d" */
    0x2, 0x4, 0x9, 0xf4, 0x30, 0x60, 0xc1, 0x82,
    0x8d, 0xe8,

    /* U+0065 "e" */
    0x3c, 0x8e, 0xf, 0xf8, 0x10, 0x10, 0x9f,

    /* U+0066 "f" */
    0x3a, 0x11, 0xe4, 0x21, 0x8, 0x42, 0x10,

    /* U+0067 "g" */
    0x3e, 0x86, 0xc, 0x18, 0x30, 0x51, 0xbd, 0x2,
    0x9, 0xe0,

    /* U+0068 "h" */
    0x82, 0x8, 0x2e, 0xc6, 0x18, 0x61, 0x86, 0x18,
    0x40,

    /* U+0069 "i" */
    0xbf, 0xc0,

    /* U+006A "j" */
    0x10, 0x31, 0x11, 0x11, 0x11, 0x11, 0xe0,

    /* U+006B "k" */
    0x81, 0x2, 0x4, 0x69, 0x96, 0x38, 0x70, 0x91,
    0x12, 0x30,

    /* U+006C "l" */
    0xff, 0xe0,

    /* U+006D "m" */
    0xb9, 0xd8, 0xc6, 0x10, 0xc2, 0x18, 0x43, 0x8,
    0x61, 0xc, 0x21,

    /* U+006E "n" */
    0xbb, 0x18, 0x61, 0x86, 0x18, 0x61,

    /* U+006F "o" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x51, 0x1c,

    /* U+0070 "p" */
    0xbd, 0x8a, 0xc, 0x18, 0x30, 0x61, 0x7c, 0x81,
    0x2, 0x0,

    /* U+0071 "q" */
    0x3e, 0x86, 0xc, 0x18, 0x30, 0x51, 0xbd, 0x2,
    0x4, 0x8,

    /* U+0072 "r" */
    0xbc, 0x88, 0x88, 0x88,

    /* U+0073 "s" */
    0x7e, 0x18, 0x3c, 0x3c, 0x18, 0x7e,

    /* U+0074 "t" */
    0x42, 0x3e, 0x84, 0x21, 0x8, 0x41, 0xc0,

    /* U+0075 "u" */
    0x86, 0x18, 0x61, 0x86, 0x18, 0xdd,

    /* U+0076 "v" */
    0x43, 0x42, 0x62, 0x26, 0x24, 0x1c, 0x18, 0x18,

    /* U+0077 "w" */
    0x88, 0xe3, 0x39, 0xcb, 0x52, 0x52, 0x94, 0xe6,
    0x31, 0x84,

    /* U+0078 "x" */
    0x43, 0x26, 0x34, 0x18, 0x18, 0x3c, 0x26, 0x43,

    /* U+0079 "y" */
    0x43, 0x42, 0x62, 0x24, 0x34, 0x1c, 0x18, 0x18,
    0x10, 0x10, 0x30,

    /* U+007A "z" */
    0xfc, 0x31, 0x84, 0x21, 0x8c, 0x3f,

    /* U+007B "{" */
    0xe, 0x20, 0x40, 0x81, 0x2, 0x8, 0x60, 0x30,
    0x20, 0x40, 0x81, 0x1, 0xc0,

    /* U+007C "|" */
    0xff, 0xfc,

    /* U+007D "}" */
    0xe0, 0x20, 0x40, 0x81, 0x2, 0x2, 0x3, 0x8,
    0x20, 0x40, 0x81, 0x1c, 0x0,

    /* U+007E "~" */
    0x70, 0xcc, 0x63, 0x70, 0xe0
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
    {.bitmap_index = 17, .adv_w = 142, .box_w = 7, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 30, .adv_w = 241, .box_w = 13, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 47, .adv_w = 163, .box_w = 10, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 60, .adv_w = 60, .box_w = 2, .box_h = 4, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 61, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 68, .adv_w = 102, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 75, .adv_w = 142, .box_w = 7, .box_h = 6, .ofs_x = 1, .ofs_y = 5},
    {.bitmap_index = 81, .adv_w = 183, .box_w = 9, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 90, .adv_w = 81, .box_w = 3, .box_h = 5, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 92, .adv_w = 102, .box_w = 4, .box_h = 1, .ofs_x = 1, .ofs_y = 4},
    {.bitmap_index = 93, .adv_w = 81, .box_w = 1, .box_h = 2, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 94, .adv_w = 102, .box_w = 6, .box_h = 13, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 104, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 113, .adv_w = 142, .box_w = 6, .box_h = 10, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 121, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 130, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 139, .adv_w = 142, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 149, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 158, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 167, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 176, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 185, .adv_w = 142, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 194, .adv_w = 102, .box_w = 1, .box_h = 8, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 195, .adv_w = 102, .box_w = 3, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 200, .adv_w = 183, .box_w = 8, .box_h = 8, .ofs_x = 2, .ofs_y = 1},
    {.bitmap_index = 208, .adv_w = 183, .box_w = 8, .box_h = 4, .ofs_x = 2, .ofs_y = 3},
    {.bitmap_index = 212, .adv_w = 183, .box_w = 8, .box_h = 8, .ofs_x = 2, .ofs_y = 1},
    {.bitmap_index = 220, .adv_w = 122, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 228, .adv_w = 224, .box_w = 12, .box_h = 11, .ofs_x = 1, .ofs_y = -1},
    {.bitmap_index = 245, .adv_w = 153, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 257, .adv_w = 154, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 266, .adv_w = 156, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 278, .adv_w = 173, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 288, .adv_w = 142, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 296, .adv_w = 129, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 304, .adv_w = 174, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 316, .adv_w = 168, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 326, .adv_w = 94, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 331, .adv_w = 102, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 338, .adv_w = 155, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 348, .adv_w = 125, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 356, .adv_w = 189, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 368, .adv_w = 168, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 378, .adv_w = 176, .box_w = 9, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 390, .adv_w = 135, .box_w = 6, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 398, .adv_w = 176, .box_w = 9, .box_h = 13, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 413, .adv_w = 156, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 423, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 433, .adv_w = 138, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 445, .adv_w = 164, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 455, .adv_w = 153, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 467, .adv_w = 221, .box_w = 12, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 482, .adv_w = 153, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 494, .adv_w = 138, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 506, .adv_w = 153, .box_w = 8, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 516, .adv_w = 102, .box_w = 3, .box_h = 14, .ofs_x = 2, .ofs_y = -3},
    {.bitmap_index = 522, .adv_w = 102, .box_w = 7, .box_h = 13, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 534, .adv_w = 102, .box_w = 3, .box_h = 14, .ofs_x = 2, .ofs_y = -3},
    {.bitmap_index = 540, .adv_w = 183, .box_w = 9, .box_h = 6, .ofs_x = 1, .ofs_y = 5},
    {.bitmap_index = 547, .adv_w = 142, .box_w = 9, .box_h = 1, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 549, .adv_w = 142, .box_w = 3, .box_h = 3, .ofs_x = 2, .ofs_y = 9},
    {.bitmap_index = 551, .adv_w = 135, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 557, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 567, .adv_w = 117, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 573, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 583, .adv_w = 133, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 590, .adv_w = 79, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 597, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 607, .adv_w = 142, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 616, .adv_w = 61, .box_w = 1, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 618, .adv_w = 77, .box_w = 4, .box_h = 13, .ofs_x = -1, .ofs_y = -3},
    {.bitmap_index = 625, .adv_w = 133, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 635, .adv_w = 61, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 637, .adv_w = 218, .box_w = 11, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 648, .adv_w = 142, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 654, .adv_w = 136, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 661, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 671, .adv_w = 140, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 681, .adv_w = 96, .box_w = 4, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 685, .adv_w = 117, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 691, .adv_w = 88, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 698, .adv_w = 142, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 704, .adv_w = 133, .box_w = 8, .box_h = 8, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 712, .adv_w = 183, .box_w = 10, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 722, .adv_w = 133, .box_w = 8, .box_h = 8, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 730, .adv_w = 133, .box_w = 8, .box_h = 11, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 741, .adv_w = 118, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 747, .adv_w = 142, .box_w = 7, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 760, .adv_w = 102, .box_w = 1, .box_h = 14, .ofs_x = 3, .ofs_y = -3},
    {.bitmap_index = 762, .adv_w = 142, .box_w = 7, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 775, .adv_w = 183, .box_w = 9, .box_h = 4, .ofs_x = 1, .ofs_y = 3}
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
    .line_height = 15,          /*The maximum line height required by the font*/
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

