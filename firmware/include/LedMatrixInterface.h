#ifndef __LedMatrixInterface_H
#define __LedMatrixInterface_H

#include "stdint.h"

class LedMatrixInterface {
public:
    virtual void setPixelColor(int pixel_num, uint8_t r, uint8_t g, uint8_t b)=0;
};

#endif // __LedMatrixInterface_H