#include "NeoPixelSpy.h"

#include "stdio.h"
#include "string.h"

// every 2 lines are reverted due to the matrix wirering
const char *letters_on_the_face = "...."
                                "RHUMAIFUOWZ"
                                "ZANIERBEUFI"
                                "LEINUNITHCA"
                                "SACHSISIBNI"
                                "TXIFUFIREIV"
                                "EISZWOISDRU"
                                "MPEIBUAHOBA"
                                "ZWANZGSIVOR"
                                "AAZFBUTREIV"
                                "ESKISCHAFUF";

void NeoPixelSpy::init(uint8_t pixel_nbr) {
    this->pixel_nbr = pixel_nbr;
}

void NeoPixelSpy::setPixelColor(uint8_t pixel_num, uint32_t rgb) {
    this->pixels[pixel_num] = rgb;
}

void NeoPixelSpy::setPixelColor(uint8_t pixel_num, uint8_t r, uint8_t g, uint8_t b) {
    this->pixels[pixel_num] = (r<<16) + (g<<8) + b;
}

void NeoPixelSpy::clear() {
    for (int i=0; i<MAX_NBR_OF_PIXEL; i++) {
        this->pixels[i] = 0;
    }
}

uint32_t NeoPixelSpy::getPixel(uint8_t pixel_num) {
    return this->pixels[pixel_num];
}

bool NeoPixelSpy::outOfBoundDetected() {
    for (int i=this->pixel_nbr; i<MAX_NBR_OF_PIXEL; i++) {
        if (this->pixels[i] != 0) {
            return true;
        }
    }

    return false;
}

char* NeoPixelSpy::toString() {
    memset(this->pixelToString, 0, sizeof(this->pixelToString));
    for (int y=this->pixel_nbr-1; y>4; y=y-11) {
        for (int x=0; x<11; x++) {
            uint8_t pixel_idx = 0;
            if (y%2 == 0) {
                pixel_idx = y-x;
            } else {
                pixel_idx = y-(10-x);
            }
            // printf("testing %d, x %d, y %d\n", pixel_idx, x, y);
            if (this->pixels[pixel_idx] != 0) {
                // printf("pixel %d x %d, y %d is set\n",pixel_idx, x, y);
                strncat(this->pixelToString, &letters_on_the_face[pixel_idx], 1);
            }
        }
    }
    return this->pixelToString;
}