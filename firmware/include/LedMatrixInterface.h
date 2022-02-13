#ifndef __LedMatrixInterface_H
#define __LedMatrixInterface_H

#include "stdint.h"

class LedMatrixInterface {
public:
    virtual void setPixelColor(uint8_t pixel_num, uint8_t r, uint8_t g, uint8_t b)=0;
    virtual void setPixelColor(uint8_t pixel_num, uint32_t rgb)=0;
    virtual void clear()=0;
};

#endif // __LedMatrixInterface_H