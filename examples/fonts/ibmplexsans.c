/*******************************************************************************
 * Size: 14 px
 * Bpp: 1
 * Opts: --bpp 1 --size 14 -o ibmplexsans.c --format lvgl --font IBMPlexSans-VariableFont_wdth,wght.ttf -r 0x0-0x7F
 ******************************************************************************/

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif

#ifndef IBMPLEXSANS
#define IBMPLEXSANS 1
#endif

#if IBMPLEXSANS

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
    0x0,

    /* U+0021 "!" */
    0xff, 0x60,

    /* U+0022 "\"" */
    0xb6, 0xd0,

    /* U+0023 "#" */
    0x12, 0x22, 0x24, 0xff, 0x24, 0x24, 0x24, 0xff,
    0x44, 0x48, 0x48,

    /* U+0024 "$" */
    0x10, 0xfb, 0x5c, 0x89, 0x1e, 0xf, 0xb, 0x13,
    0x27, 0x5b, 0xc1, 0x0,

    /* U+0025 "%" */
    0x60, 0x92, 0x12, 0x44, 0x49, 0x9, 0x20, 0xc9,
    0x82, 0x48, 0x49, 0x11, 0x24, 0x24, 0x83, 0x0,

    /* U+0026 "&" */
    0x38, 0x44, 0x44, 0x4c, 0x38, 0x73, 0x9a, 0x8e,
    0x84, 0xc6, 0x7b,

    /* U+0027 "'" */
    0xf0,

    /* U+0028 "(" */
    0x24, 0x48, 0x88, 0x88, 0x88, 0x44, 0x23,

    /* U+0029 ")" */
    0x42, 0x21, 0x11, 0x11, 0x11, 0x22, 0x4c,

    /* U+002A "*" */
    0x21, 0x3e, 0xe5, 0x0,

    /* U+002B "+" */
    0x10, 0x20, 0x47, 0xf1, 0x2, 0x4, 0x0,

    /* U+002C "," */
    0x55,

    /* U+002D "-" */
    0xf0,

    /* U+002E "." */
    0xc0,

    /* U+002F "/" */
    0x8, 0x84, 0x21, 0x10, 0x84, 0x42, 0x10, 0x88,
    0x40,

    /* U+0030 "0" */
    0x38, 0x8a, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x82,
    0x88, 0xe0,

    /* U+0031 "1" */
    0x18, 0x51, 0x20, 0x40, 0x81, 0x2, 0x4, 0x8,
    0x11, 0xf8,

    /* U+0032 "2" */
    0x7b, 0x38, 0x41, 0x4, 0x21, 0x8c, 0x63, 0xf,
    0xc0,

    /* U+0033 "3" */
    0x79, 0x10, 0x41, 0xc, 0xc0, 0xc1, 0x7, 0x37,
    0x80,

    /* U+0034 "4" */
    0xc, 0xc, 0x14, 0x14, 0x24, 0x24, 0x44, 0x44,
    0xff, 0x4, 0x4,

    /* U+0035 "5" */
    0xfa, 0x8, 0x20, 0xbb, 0x30, 0x41, 0x6, 0x27,
    0x80,

    /* U+0036 "6" */
    0x10, 0x84, 0x10, 0xbb, 0x38, 0x61, 0x85, 0x37,
    0x80,

    /* U+0037 "7" */
    0xff, 0xa, 0x10, 0x60, 0x81, 0x4, 0x8, 0x30,
    0x40, 0x80,

    /* U+0038 "8" */
    0x79, 0x9a, 0x14, 0x2c, 0xcf, 0x31, 0x41, 0x83,
    0x8d, 0xf0,

    /* U+0039 "9" */
    0x7b, 0x28, 0x61, 0x87, 0x37, 0x42, 0x8, 0x42,
    0x0,

    /* U+003A ":" */
    0xc3,

    /* U+003B ";" */
    0x50, 0x5, 0x50,

    /* U+003C "<" */
    0x0, 0x31, 0x98, 0x83, 0x3, 0x6, 0x4,

    /* U+003D "=" */
    0xfe, 0x0, 0x7, 0xf0,

    /* U+003E ">" */
    0x3, 0x6, 0x6, 0x4, 0x33, 0x18, 0x80,

    /* U+003F "?" */
    0x76, 0x62, 0x11, 0x98, 0x80, 0x1, 0x8,

    /* U+0040 "@" */
    0x1f, 0x6, 0x11, 0x1, 0x27, 0x99, 0x13, 0x22,
    0x64, 0x4c, 0x89, 0x91, 0x71, 0xf9, 0x0, 0x10,
    0x1, 0xf8,

    /* U+0041 "A" */
    0x8, 0xa, 0x5, 0x2, 0x83, 0x61, 0x10, 0x88,
    0xfe, 0x41, 0x20, 0xb0, 0x60,

    /* U+0042 "B" */
    0xf9, 0xa, 0x14, 0x28, 0x5f, 0x21, 0xc1, 0x83,
    0x7, 0xf0,

    /* U+0043 "C" */
    0x3c, 0x8e, 0xc, 0x8, 0x10, 0x20, 0x40, 0x82,
    0x8c, 0xf0,

    /* U+0044 "D" */
    0xf9, 0xa, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x83,
    0xb, 0xe0,

    /* U+0045 "E" */
    0xfe, 0x8, 0x20, 0x83, 0xf8, 0x20, 0x82, 0xf,
    0xc0,

    /* U+0046 "F" */
    0xfe, 0x8, 0x20, 0x83, 0xe8, 0x20, 0x82, 0x8,
    0x0,

    /* U+0047 "G" */
    0x3c, 0x42, 0x40, 0x80, 0x80, 0x87, 0x81, 0x81,
    0x41, 0x43, 0x3d,

    /* U+0048 "H" */
    0x83, 0x6, 0xc, 0x18, 0x3f, 0xe0, 0xc1, 0x83,
    0x6, 0x8,

    /* U+0049 "I" */
    0xf4, 0x44, 0x44, 0x44, 0x44, 0xf0,

    /* U+004A "J" */
    0x78, 0x42, 0x10, 0x84, 0x21, 0x8c, 0x5c,

    /* U+004B "K" */
    0x85, 0x1a, 0x64, 0x8a, 0x1e, 0x34, 0x44, 0x8d,
    0xa, 0x18,

    /* U+004C "L" */
    0x84, 0x21, 0x8, 0x42, 0x10, 0x84, 0x3e,

    /* U+004D "M" */
    0xc1, 0xe0, 0xf8, 0xf4, 0x5b, 0x6c, 0xa6, 0x63,
    0x11, 0x80, 0xc0, 0x60, 0x20,

    /* U+004E "N" */
    0x83, 0x87, 0x8d, 0x1b, 0x32, 0x66, 0xc5, 0x8f,
    0xe, 0x8,

    /* U+004F "O" */
    0x3c, 0x42, 0x83, 0x81, 0x81, 0x81, 0x81, 0x81,
    0xc3, 0x42, 0x3c,

    /* U+0050 "P" */
    0xfa, 0x18, 0x61, 0x87, 0xe8, 0x20, 0x82, 0x8,
    0x0,

    /* U+0051 "Q" */
    0x3c, 0x42, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81,
    0x42, 0x6e, 0x1c, 0x8, 0xc,

    /* U+0052 "R" */
    0xfa, 0x18, 0x61, 0x87, 0xe9, 0xa2, 0x8a, 0x18,
    0x40,

    /* U+0053 "S" */
    0x7a, 0x38, 0x20, 0xe1, 0xe0, 0xc1, 0x86, 0x37,
    0x80,

    /* U+0054 "T" */
    0xfe, 0x20, 0x40, 0x81, 0x2, 0x4, 0x8, 0x10,
    0x20, 0x40,

    /* U+0055 "U" */
    0x83, 0x6, 0xc, 0x18, 0x30, 0x60, 0xc1, 0x82,
    0x89, 0xf0,

    /* U+0056 "V" */
    0xc1, 0x41, 0x43, 0x62, 0x22, 0x26, 0x24, 0x14,
    0x14, 0x18, 0x18,

    /* U+0057 "W" */
    0xc6, 0x14, 0x61, 0x46, 0x34, 0x52, 0x45, 0x26,
    0x92, 0x29, 0x22, 0x8e, 0x28, 0xc3, 0xc, 0x10,
    0xc0,

    /* U+0058 "X" */
    0x41, 0x62, 0x26, 0x34, 0x1c, 0x18, 0x1c, 0x34,
    0x26, 0x62, 0x41,

    /* U+0059 "Y" */
    0x83, 0x8d, 0x13, 0x62, 0x87, 0x4, 0x8, 0x10,
    0x20, 0x40,

    /* U+005A "Z" */
    0xfe, 0xc, 0x10, 0x41, 0x82, 0x8, 0x30, 0x41,
    0x3, 0xf8,

    /* U+005B "[" */
    0xea, 0xaa, 0xaa, 0xb0,

    /* U+005C "\\" */
    0x88, 0x8c, 0x44, 0x42, 0x22, 0x21, 0x11,

    /* U+005D "]" */
    0xd5, 0x55, 0x55, 0x70,

    /* U+005E "^" */
    0x30, 0x71, 0x22, 0x68, 0x50, 0x40,

    /* U+005F "_" */
    0xfe,

    /* U+0060 "`" */
    0xa4,

    /* U+0061 "a" */
    0x79, 0x8, 0x13, 0xe8, 0x50, 0xa3, 0x3b,

    /* U+0062 "b" */
    0x82, 0x8, 0x2e, 0xca, 0x18, 0x61, 0x86, 0x2f,
    0x80,

    /* U+0063 "c" */
    0x79, 0x38, 0x20, 0x82, 0x4, 0xde,

    /* U+0064 "d" */
    0x4, 0x10, 0x5d, 0x4e, 0x18, 0x61, 0x85, 0x17,
    0xc0,

    /* U+0065 "e" */
    0x79, 0x38, 0x7f, 0x82, 0x4, 0xde,

    /* U+0066 "f" */
    0x74, 0x4f, 0x44, 0x44, 0x44, 0x40,

    /* U+0067 "g" */
    0x6, 0xf2, 0x14, 0x28, 0x4f, 0x20, 0x40, 0x7d,
    0x6, 0xb, 0xe0,

    /* U+0068 "h" */
    0x82, 0x8, 0x2e, 0xce, 0x18, 0x61, 0x86, 0x18,
    0x40,

    /* U+0069 "i" */
    0x9f, 0xe0,

    /* U+006A "j" */
    0x41, 0x55, 0x55, 0x70,

    /* U+006B "k" */
    0x82, 0x8, 0x22, 0x92, 0x8e, 0x3c, 0x92, 0x28,
    0xc0,

    /* U+006C "l" */
    0xaa, 0xaa, 0xac,

    /* U+006D "m" */
    0xb7, 0x64, 0x62, 0x31, 0x18, 0x8c, 0x46, 0x23,
    0x11,

    /* U+006E "n" */
    0xbb, 0x38, 0x61, 0x86, 0x18, 0x61,

    /* U+006F "o" */
    0x79, 0x28, 0x61, 0x86, 0x14, 0x9e,

    /* U+0070 "p" */
    0xbb, 0x28, 0x61, 0x86, 0x18, 0xbe, 0x82, 0x8,
    0x0,

    /* U+0071 "q" */
    0x75, 0x38, 0x61, 0x86, 0x14, 0x5f, 0x4, 0x10,
    0x40,

    /* U+0072 "r" */
    0xbc, 0x88, 0x88, 0x88,

    /* U+0073 "s" */
    0x74, 0x61, 0xc3, 0x86, 0x2e,

    /* U+0074 "t" */
    0x44, 0xf4, 0x44, 0x44, 0x47,

    /* U+0075 "u" */
    0x86, 0x18, 0x61, 0x86, 0x1c, 0x5f,

    /* U+0076 "v" */
    0xc4, 0x89, 0x12, 0x62, 0x85, 0xa, 0x8,

    /* U+0077 "w" */
    0x44, 0x53, 0x14, 0xa5, 0x2a, 0x2a, 0x8c, 0xa3,
    0x18, 0xc6,

    /* U+0078 "x" */
    0x44, 0xc8, 0xa0, 0x81, 0x85, 0x19, 0x22,

    /* U+0079 "y" */
    0x87, 0x14, 0x52, 0x68, 0xa3, 0xc, 0x10, 0x86,
    0x0,

    /* U+007A "z" */
    0xf8, 0xc4, 0x42, 0x22, 0x1f,

    /* U+007B "{" */
    0x32, 0x22, 0x22, 0xc2, 0x22, 0x22, 0x23,

    /* U+007C "|" */
    0xff, 0xfc,

    /* U+007D "}" */
    0xc4, 0x44, 0x44, 0x34, 0x44, 0x44, 0x4c,

    /* U+007E "~" */
    0xe0, 0x38
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 53, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 1, .adv_w = 64, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 3, .adv_w = 94, .box_w = 3, .box_h = 4, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 5, .adv_w = 160, .box_w = 8, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 16, .adv_w = 134, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = -1},
    {.bitmap_index = 28, .adv_w = 208, .box_w = 11, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 44, .adv_w = 155, .box_w = 8, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 55, .adv_w = 54, .box_w = 1, .box_h = 4, .ofs_x = 1, .ofs_y = 7},
    {.bitmap_index = 56, .adv_w = 75, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 63, .adv_w = 75, .box_w = 4, .box_h = 14, .ofs_x = -1, .ofs_y = -3},
    {.bitmap_index = 70, .adv_w = 101, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 6},
    {.bitmap_index = 74, .adv_w = 134, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 1},
    {.bitmap_index = 81, .adv_w = 61, .box_w = 2, .box_h = 4, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 82, .adv_w = 89, .box_w = 4, .box_h = 1, .ofs_x = 1, .ofs_y = 4},
    {.bitmap_index = 83, .adv_w = 61, .box_w = 1, .box_h = 2, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 84, .adv_w = 86, .box_w = 5, .box_h = 14, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 93, .adv_w = 134, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 103, .adv_w = 134, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 113, .adv_w = 134, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 122, .adv_w = 134, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 131, .adv_w = 134, .box_w = 8, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 142, .adv_w = 134, .box_w = 6, .box_h = 11, .ofs_x = 2, .ofs_y = 0},
    {.bitmap_index = 151, .adv_w = 134, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 160, .adv_w = 134, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 170, .adv_w = 134, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 180, .adv_w = 134, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 189, .adv_w = 65, .box_w = 1, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 190, .adv_w = 65, .box_w = 2, .box_h = 10, .ofs_x = 0, .ofs_y = -2},
    {.bitmap_index = 193, .adv_w = 134, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 200, .adv_w = 134, .box_w = 7, .box_h = 4, .ofs_x = 1, .ofs_y = 3},
    {.bitmap_index = 204, .adv_w = 134, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 211, .adv_w = 107, .box_w = 5, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 218, .adv_w = 200, .box_w = 11, .box_h = 13, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 236, .adv_w = 144, .box_w = 9, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 249, .adv_w = 146, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 259, .adv_w = 139, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 269, .adv_w = 150, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 279, .adv_w = 131, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 288, .adv_w = 125, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 297, .adv_w = 156, .box_w = 8, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 308, .adv_w = 158, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 318, .adv_w = 90, .box_w = 4, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 324, .adv_w = 114, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 331, .adv_w = 142, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 341, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 348, .adv_w = 182, .box_w = 9, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 361, .adv_w = 158, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 371, .adv_w = 159, .box_w = 8, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 382, .adv_w = 136, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 391, .adv_w = 159, .box_w = 8, .box_h = 13, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 404, .adv_w = 143, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 413, .adv_w = 130, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 422, .adv_w = 128, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 432, .adv_w = 152, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 442, .adv_w = 136, .box_w = 8, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 453, .adv_w = 200, .box_w = 12, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 470, .adv_w = 137, .box_w = 8, .box_h = 11, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 481, .adv_w = 133, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 491, .adv_w = 130, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 501, .adv_w = 71, .box_w = 2, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 505, .adv_w = 86, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 512, .adv_w = 71, .box_w = 2, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 516, .adv_w = 134, .box_w = 7, .box_h = 6, .ofs_x = 1, .ofs_y = 5},
    {.bitmap_index = 522, .adv_w = 127, .box_w = 7, .box_h = 1, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 523, .adv_w = 134, .box_w = 2, .box_h = 3, .ofs_x = 3, .ofs_y = 9},
    {.bitmap_index = 524, .adv_w = 120, .box_w = 7, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 531, .adv_w = 130, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 540, .adv_w = 113, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 546, .adv_w = 130, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 555, .adv_w = 123, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 561, .adv_w = 73, .box_w = 4, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 567, .adv_w = 118, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 578, .adv_w = 127, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 587, .adv_w = 56, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 589, .adv_w = 56, .box_w = 2, .box_h = 14, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 593, .adv_w = 118, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 602, .adv_w = 61, .box_w = 2, .box_h = 11, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 605, .adv_w = 196, .box_w = 9, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 614, .adv_w = 127, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 620, .adv_w = 125, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 626, .adv_w = 130, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 635, .adv_w = 130, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 644, .adv_w = 82, .box_w = 4, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 648, .adv_w = 109, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 653, .adv_w = 79, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 658, .adv_w = 127, .box_w = 6, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 664, .adv_w = 110, .box_w = 7, .box_h = 8, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 671, .adv_w = 172, .box_w = 10, .box_h = 8, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 681, .adv_w = 114, .box_w = 7, .box_h = 8, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 688, .adv_w = 112, .box_w = 6, .box_h = 11, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 697, .adv_w = 104, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 702, .adv_w = 77, .box_w = 4, .box_h = 14, .ofs_x = 0, .ofs_y = -3},
    {.bitmap_index = 709, .adv_w = 70, .box_w = 1, .box_h = 14, .ofs_x = 2, .ofs_y = -3},
    {.bitmap_index = 711, .adv_w = 77, .box_w = 4, .box_h = 14, .ofs_x = 1, .ofs_y = -3},
    {.bitmap_index = 718, .adv_w = 134, .box_w = 7, .box_h = 2, .ofs_x = 1, .ofs_y = 4}
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
    .kern_dsc = NULL,
    .kern_scale = 0,
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
const lv_font_t ibmplexsans = {
#else
lv_font_t ibmplexsans = {
#endif
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 15,          /*The maximum line height required by the font*/
    .base_line = 3,             /*Baseline measured from the bottom of the line*/
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



#endif /*#if IBMPLEXSANS*/

