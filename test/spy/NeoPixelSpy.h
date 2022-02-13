#ifndef __NEO_PIXEL_SPY
#define __NEO_PIXEL_SPY

#include "stdint.h"

#include "LedMatrixInterface.h"

#define MAX_NBR_OF_PIXEL UINT8_MAX

class NeoPixelSpy: public LedMatrixInterface {
public:
    void init(uint8_t pixel_nbr);
    void setPixelColor(uint8_t pixel_num, uint32_t rgb);
    void setPixelColor(uint8_t pixel_num, uint8_t r, uint8_t g, uint8_t b);
    void clear();

    uint32_t getPixel(uint8_t pixel_num);
    bool outOfBoundDetected();
    char* toString();
private:
    uint32_t pixels[MAX_NBR_OF_PIXEL];
    uint8_t pixel_nbr;

    char pixelToString[MAX_NBR_OF_PIXEL];
};

#endif // __NEO_PIXEL_SPY