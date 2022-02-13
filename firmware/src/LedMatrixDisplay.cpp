#include "LedMatrixDisplay.h"

#include "stdio.h"

// TODO: Make this runtime configurable

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

#define MIN_5 1
#define MIN_10 2
#define MIN_15 3
#define MIN_20 4
#define MIN_30 5
const uint8_t index_to_minutes_lut[2][6][3] = {
{	// Bärn
    // start_x, start_y, length
    {8, 9, 3}, // uhr
    {8, 0, 3}, // fÜf
    {8, 1, 3}, // zää
    {0, 1, 6}, // viertu
    {0, 2, 6}, // zwänzg
    {3, 3, 5}, // haubi
},
{	// Wallis
    // start_x, start_y, length
    {8, 9, 3}, // uhr
    {8, 0, 3}, // fÜf
    {8, 1, 3}, // zäh
    {0, 1, 7}, // viertel
    {0, 2, 6}, // zwenzg
    {3, 3, 5}, // halbi
}};

const uint8_t index_to_preposition_lut[2][2][3] = {
{	// Bärn
    // start_x, start_y, length
    {0, 3, 2}, // ab
    {8, 2, 3}, // vor
},
{	// Wallis
    // start_x, start_y, length
    {0, 3, 2}, // ab
    {8, 2, 3}, // vor
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
    this->dialect = DIALECT_BARN;
}

void LedMatrixDisplay::setDialect(DisplayDialect_t dialect) {
    this->dialect = dialect;
}

void LedMatrixDisplay::turnOn(uint8_t position, DisplayColor_t color) {
    this->ledMatrix->setPixelColor(position, color);
}

void LedMatrixDisplay::turnOnAll(DisplayColor_t color) {
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

void LedMatrixDisplay::displayTime(uint8_t hour, uint8_t minutes) {
    this->ledMatrix->clear();

    // Display hour
    int hour_to_display = hour;
    if (minutes >= 25) {
        hour_to_display = hour+1;
    }
    // 24 -> 12
    if (hour_to_display > 12) {
        hour_to_display = hour_to_display - 12 ;
    }
    // midnight
    if (hour_to_display == 0) {
        hour_to_display = 12;
    }
    int hour_idx = hour_to_display-1;

    this->displayFromLut(index_to_hours_lut[this->dialect], hour_idx, WHITE);

    // Display minutes
    int minutes_word=0;
    if (minutes < 5) {
        // Es isch
        displayFromLut(index_to_word_lut[this->dialect], 0, WHITE);
        displayFromLut(index_to_word_lut[this->dialect], 1, WHITE);
        // uhr
        minutes_word = minutes/5;
        displayFromLut(index_to_minutes_lut[this->dialect], minutes_word, WHITE);
    }else if (minutes < 25) {
        // uhr, füf, zää, zwanzg ab
        minutes_word = minutes/5;
        displayFromLut(index_to_minutes_lut[this->dialect], minutes_word, WHITE);
        displayFromLut(index_to_preposition_lut[this->dialect], 0, WHITE);
    } else if (minutes < 30) {
        // füf vor halbi h+1
        displayFromLut(index_to_minutes_lut[this->dialect], MIN_5, WHITE);
        displayFromLut(index_to_preposition_lut[this->dialect], 1, WHITE);
        displayFromLut(index_to_minutes_lut[this->dialect], MIN_30, WHITE);

    } else if (minutes < 35) {
        // halbi h+1
        displayFromLut(index_to_minutes_lut[this->dialect], MIN_30, WHITE);
    } else if (minutes < 40) {
        // füf ab halbi h+1
        displayFromLut(index_to_minutes_lut[this->dialect], MIN_5, WHITE);
        displayFromLut(index_to_preposition_lut[this->dialect], 0, WHITE);
        displayFromLut(index_to_minutes_lut[this->dialect], MIN_30, WHITE);
    } else {
        // füf, zää, zwanzg vor
        minutes_word = (60 - minutes-1)/5 +1;
        displayFromLut(index_to_minutes_lut[this->dialect], minutes_word, WHITE);
        displayFromLut(index_to_preposition_lut[this->dialect], 1, WHITE);
    }
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

void LedMatrixDisplay::setPixel(uint8_t x, uint8_t y, DisplayColor_t color) {
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

    this->ledMatrix->setPixelColor(pixel_num,color);
}

void LedMatrixDisplay::displayFromLut(const uint8_t lut[][3], uint8_t ele, DisplayColor_t color) {
    for (int x=0; x<lut[ele][2]; x++) {
        this->setPixel(lut[ele][0]+x, lut[ele][1], color);
    }
}