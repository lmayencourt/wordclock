#include "LedMatrixDisplay.h"

void LedMatrixDisplay::init(LedMatrixInterface *ledMatrix) {
    this->ledMatrix = ledMatrix;
};

void LedMatrixDisplay::turnOn(uint8_t position, uint8_t color) {
    // ledMatrix[position] = color;
};