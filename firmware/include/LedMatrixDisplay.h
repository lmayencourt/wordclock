#ifndef __LED_MATRIX_DISPLAY_H
#define __LED_MATRIX_DISPLAY_H

#include "stdint.h"

#include "LedMatrixInterface.h"

#define LEDMATRIX_STATE_DOT_IDX 0
#define LEDMATRIX_STATE_DOT_NBR 4

#define DISPLAY_WIDTH 11
#define DISPLAY_HEIGTH 10

typedef enum {
    OFF  = 0x000000,
    RED = 0x00ff0000,
    GREEN = 0x0000ff00,
    BLUE = 0x000000ff,
    WHITE = 0xffffff,
} DisplayColor_t;

enum {
    STATE_STARTUP,
    STATE_CONFIG,
    STATE_ERROR,
};

typedef enum {
    DIALECT_BARN=0,
    DIALECT_WALLIS=1,
} DisplayDialect_t;

class LedMatrixDisplay {
public:
    void init(LedMatrixInterface *ledMatrix, uint8_t pixel_nbr);
    void setDialect(DisplayDialect_t dialect);
    void turnOn(uint8_t position, DisplayColor_t color);
    void turnOnAll(DisplayColor_t color);
    void turnOff(uint8_t position);
    void turnOffAll();
    void displayState(uint8_t state);
    void displayTime(uint8_t hour, uint8_t minutes);
private:
    LedMatrixInterface *ledMatrix;
    uint8_t pixel_nbr;
    DisplayDialect_t dialect;

    void setPixel(uint8_t x, uint8_t y, DisplayColor_t color);
    void displayFromLut(const uint8_t lut[][3], uint8_t ele, DisplayColor_t color);

    void displayStartup();
    void displayConfig();
    void displayError();
};

#endif // __LED_MATRIX_DISPLAY_H