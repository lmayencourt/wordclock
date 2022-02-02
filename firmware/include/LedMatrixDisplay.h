#ifndef __LED_MATRIX_DISPLAY_H
#define __LED_MATRIX_DISPLAY_H

#include "stdint.h"

#include "LedMatrixInterface.h"

enum {
    WHITE = 0xff,
};

class LedMatrixDisplay {
public:
    void init(LedMatrixInterface *ledMatrix);
    void turnOn(uint8_t position, uint8_t color);
private:
    LedMatrixInterface *ledMatrix;
};

#endif // __LED_MATRIX_DISPLAY_H