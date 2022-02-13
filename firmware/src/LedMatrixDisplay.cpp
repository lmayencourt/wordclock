#include "LedMatrixDisplay.h"

#include "stdio.h"

const uint8_t index_to_hours_lut[2][12][3] = {
{	// Bärn
    // start_x, start_y, length
    {0, 4, 3}, // eis
    {3, 4, 4}, // zwöi
    {8, 4, 3}, // drü
    {0, 5, 5}, // vieri
    {5, 5, 4}, // füfi
    {0, 6, 6}, // sächsi
    {6, 6, 5}, // sibni
    {0, 7, 5}, // achti
    {5, 7, 4}, // nüni
    {0, 8, 4}, // zäni
    {7, 8, 4}, // eufi
    {0, 9, 6}, // zwöufi 
},
{	// Wallis
    // start_x, start_y, length
    {0, 4, 3}, // eis
    {3, 4, 4}, // zwei
    {8, 4, 3}, // dri
    {0, 5, 5}, // vieri
    {7, 6, 4}, // füfi
    {0, 6, 7}, // sägschi
    {6, 5, 5}, // sibni
    {0, 7, 5}, // achti
    {5, 7, 4}, // nini
    {0, 8, 5}, // zähni
    {7, 8, 4}, // elfi
    {0, 9, 6}, // zwelfi
}};

const uint8_t index_to_word_lut[2][3][3] = {
{	// Bärn
    // start_x, start_y, length
    {0, 0, 2}, // es
    {3, 0, 4}, // isch
    {8, 9, 3}, // uhr
},
{	// Wallis
    // start_x, start_y, length
    {0, 0, 2}, // äs
    {3, 0, 4}, // isch
    {8, 9, 3}, // uhr
}};

void LedMatrixDisplay::init(LedMatrixInterface *ledMatrix, uint8_t pixel_nbr) {
    this->ledMatrix = ledMatrix;
    this->pixel_nbr = pixel_nbr;
}

void LedMatrixDisplay::turnOn(uint8_t position, uint32_t color) {
    this->ledMatrix->setPixelColor(position, color);
}

void LedMatrixDisplay::turnOnAll(uint32_t color) {
    for (int i=0; i<this->pixel_nbr; i++) {
        this->ledMatrix->setPixelColor(i, color);
    }
}

void LedMatrixDisplay::turnOff(uint8_t position) {
    this->ledMatrix->setPixelColor(position, OFF);
}

void LedMatrixDisplay::turnOffAll() {
    this->ledMatrix->clear();
}

void LedMatrixDisplay::displayState(uint8_t state) {
    switch(state) {
        case STATE_STARTUP:
            this->displayStartup();
            break;
        case STATE_CONFIG:
            this->displayConfig();
            break;
        case STATE_ERROR:
            this->displayError();
            break;
        default:
            this->displayError();
            break;
    }
}

void LedMatrixDisplay::displayTime(uint8_t hour, uint8_t minute) {
    this->ledMatrix->clear();
    this->displayFromLut(index_to_word_lut[0], 0, WHITE);
    this->displayFromLut(index_to_word_lut[0], 1, WHITE);
    this->displayFromLut(index_to_word_lut[0], 2, WHITE);

    uint8_t hour_idx = hour -1;
    this->displayFromLut(index_to_hours_lut[0], hour_idx, WHITE);
}

void LedMatrixDisplay::displayStartup() {
    this->ledMatrix->setPixelColor(LEDMATRIX_STATE_DOT_IDX, GREEN);
}

void LedMatrixDisplay::displayConfig() {
    for (int i=LEDMATRIX_STATE_DOT_IDX; i<(LEDMATRIX_STATE_DOT_IDX+2); i++) {
        this->ledMatrix->setPixelColor(i, BLUE);
    }
}

void LedMatrixDisplay::displayError() {
    for (int i=LEDMATRIX_STATE_DOT_IDX; i<(LEDMATRIX_STATE_DOT_IDX + LEDMATRIX_STATE_DOT_NBR); i++) {
        this->ledMatrix->setPixelColor(i, RED);
    }
}

void LedMatrixDisplay::setPixel(uint8_t x, uint8_t y, uint32_t color) {
    uint8_t corrected_x = x;
    uint8_t corrected_y = DISPLAY_HEIGTH-1-y;
    uint8_t pixel_num = 0;
    if (y%2 == 1) {
        // we are on a odd line
        corrected_x = DISPLAY_WIDTH-1-x;
    }
    pixel_num = (DISPLAY_WIDTH)*corrected_y + corrected_x;
    // minutes dots are the 4 first leds. Map them at [10,0-3]
    if (y < DISPLAY_HEIGTH) {
        pixel_num += 4;
    } else {
        pixel_num = x;
    }

    // printf("setting x %d, y %d-> %d\n", x, y, pixel_num);
    this->ledMatrix->setPixelColor(pixel_num,color);
}

void LedMatrixDisplay::displayFromLut(const uint8_t lut[][3], uint8_t ele, uint32_t color) {
    // printf("display from lut %d: %d, %d, %d\n", ele, lut[ele][0], lut[ele][1], lut[ele][2]);
    for (int x=0; x<lut[ele][2]; x++) {
        this->setPixel(lut[ele][0]+x, lut[ele][1], color);
    }
}