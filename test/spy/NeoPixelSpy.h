#ifndef __NEO_PIXEL_SPY
#define __NEO_PIXEL_SPY

#include "stdint.h"

#include "LedMatrixInterface.h"

class NoePixelSpy: public LedMatrixInterface {
public:
    void setPixelColor(int pixel_num, uint8_t r, uint8_t g, uint8_t b);

private:
    // uint32_t pixels[114];
};

#endif // __NEO_PIXEL_SPY