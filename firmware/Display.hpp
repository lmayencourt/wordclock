#ifndef __DISPLAY_HPP
#define __DISPLAY_HPP

#include <Adafruit_NeoPixel.h>

#include "time.h"

// Which pin on the Arduino is connected to the NeoPixels?
#define PIN 13 // On Trinket or Gemma, suggest changing this to 1

// How many NeoPixels are attached to the Arduino?
#define NUMPIXELS 114 // Popular NeoPixel ring size

#define DISPLAY_WIDTH 11
#define DISPLAY_HEIGTH 10

#define MAX_BRIGTHNESS 20

#define DISPLAY_TESTS_PIXELS 0
#define DISPLAY_TESTS_WORD 1
#define DISPLAY_TESTS_TIME 4

// When setting up the NeoPixel library, we tell it how many pixels,
// and which pin to use to send signals. Note that for older NeoPixel
// strips you might need to change the third parameter -- see the
// strandtest example for more information on possible values.
Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);

class Display {
private:

	int grid[DISPLAY_HEIGTH][DISPLAY_WIDTH];

	int hours[12][3] = {
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
	};

	#define MIN_5 1
	#define MIN_10 2
	#define MIN_15 3
	#define MIN_20 4
	#define MIN_30 5
	int minutes[6][3] = {
		// start_x, start_y, length
		{8, 9, 3}, // uhr
		{8, 0, 3}, // fÜf
		{8, 1, 3}, // zää
		{0, 1, 6}, // viertu
		{0, 2, 6}, // zwänzg
		{3, 3, 5}, // haubi
	};

	int preposition[2][3] = {
		// start_x, start_y, length
		{0, 3, 2}, // ab
		{8, 2, 3}, // vor
	};

	int word[3][3] = {
		// start_x, start_y, length
		{0, 0, 2}, // es
		{3, 0, 4}, // isch
		{8, 9, 3}, // uhr
	};

	int dots[4][3] = {
		// start_x, start_y, length
		{0, 10, 1}, // .
		{0, 10, 2}, // . .
		{0, 10, 3}, // . . .
		{0, 10, 4}, // . . . .
	};

public:
	void init() {
		Serial.println("Init NeoPixel\r\n");
		pixels.begin();

		pixels.clear();
		pixels.show();
  }

  	void display() {
  		pixels.show();
  	}

  	void clear() {
  		pixels.clear();
  	}

	void clearDots() {
		for (int x=0; x<4; x++)
			setPixel(0, 10+x, 0);
	}

  	void setPixel(int x, int y, int brigthness) {
		int corrected_x = x;
		int corrected_y = DISPLAY_HEIGTH-1-y;
		int pixel_num = 0;
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

		if (brigthness <= MAX_BRIGTHNESS)
			pixels.setPixelColor(pixel_num, pixels.Color(brigthness, brigthness, brigthness));
		else
			pixels.setPixelColor(pixel_num, pixels.Color(MAX_BRIGTHNESS, MAX_BRIGTHNESS, MAX_BRIGTHNESS));

  	}

	void displayFromLut(int lut[][3], int ele) {
		for (int x=0; x<lut[ele][2]; x++) {
			setPixel(lut[ele][0]+x, lut[ele][1], 10);
		}
	}

  	void test(int level) {
		int brightness = 2;
		if (level == 0) {
			// set every pixel one by one in order
			clear();
			for (int y=0; y<DISPLAY_HEIGTH; y++) {
				for (int x=0; x<DISPLAY_WIDTH; x++) {
					setPixel(x, y, brightness);
					display();
					delay(80);
					clear();
				}
			}

			clear();
			// minutes dots
			for (int x=0; x<4; x++) {
					setPixel(x, DISPLAY_HEIGTH, brightness);
					display();
					delay(80);
					clear();
			}

			// display a diagonal
			clear();
			for (int y=0; y<DISPLAY_HEIGTH; y++) {
				for (int x=0; x<DISPLAY_WIDTH; x++) {
					if (x==y)
						setPixel(x, y, brightness);
					else
						setPixel(x, y, 0);
				}
			}

			display();
			delay(1000);
		}
		if (level <= 1) {
			// display all number
			clear();
			for (int i=0; i<12; i++) {
				displayFromLut(hours, i);
				display();
				delay(750);
				clear();
			}
			// display all minutes
			clear();
			for (int i=0; i<6; i++) {
				displayFromLut(minutes, i);
				display();
				delay(750);
				clear();
			}
			// display all preposition
			clear();
			for (int i=0; i<2; i++) {
				displayFromLut(preposition, i);
				display();
				delay(750);
				clear();
			}
			// display all word
			clear();
			for (int i=0; i<3; i++) {
				displayFromLut(word, i);
				display();
				delay(750);
				clear();
			}
			// display all dots
			clear();
			for (int i=0; i<4; i++) {
				displayFromLut(dots, i);
				display();
				delay(750);
				clear();
			}
		}
		if (level <= 2) {
			// display all time
			clear();
			for (int h=0; h<12; h++) {
				for (int m=0; m<60; m++) {
					displayTime(h, m);
					delay(50);
				}
			}
		}
		if (level <= 3) {
			// display only 1 hour
			clear();
			for (int h=0; h<2; h++) {
				for (int m=0; m<60; m++) {
					displayTime(h, m);
					delay(50);
				}
			}
		}

		clear();
  	}

  	void displayTime(int hour, int min) {
  		Serial.printf("display %d:%d", hour, min);
		int brightness = 10;
		clear();

		// Display hour
		int hour_to_display = hour;
		if (min >= 25) {
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
		Serial.printf("--> h: %d, h_idx: %d | ", hour_to_display, hour_idx);
		displayFromLut(hours, hour_idx);

		// Display minutes
		int minutes_word=0;
		if (min < 5) {
			// Es isch
			displayFromLut(word, 0);
			displayFromLut(word, 1);
			// uhr
			minutes_word = min/5;
			Serial.printf(" <5 id: %d\n\r", minutes_word);
			displayFromLut(minutes, minutes_word);
		}else if (min < 25) {
			// uhr, füf, zää, zwanzg ab
			minutes_word = min/5;
			Serial.printf(" <25 id: %d\n\r", minutes_word);
			displayFromLut(minutes, minutes_word);
			displayFromLut(preposition, 0);
		} else if (min < 30) {
			// füf vor halbi h+1
			Serial.printf(" <30\n\r");
			displayFromLut(minutes, MIN_5);
			displayFromLut(preposition, 1);
			displayFromLut(minutes,MIN_30);

		} else if (min < 35) {
			// halbi h+1
			Serial.printf(" <35\n\r");
			displayFromLut(minutes,MIN_30);
		} else if (min < 40) {
			// füf ab halbi h+1
			Serial.printf(" <40\n\r");
			displayFromLut(minutes, MIN_5);
			displayFromLut(preposition, 0);
			displayFromLut(minutes,MIN_30);
		} else {
			// füf, zää, zwanzg vor
			minutes_word = (60 - min-1)/5 +1;
			Serial.printf(" >40 id: %d\n\r", minutes_word);
			displayFromLut(minutes, minutes_word);
			displayFromLut(preposition, 1);
		}
		
		// display inter'minutes
		int minutes_dot = min%5 -1; 
		displayFromLut(dots, minutes_dot);

		display();
  	}

	void displayError() {
		clear();
		pixels.setPixelColor(0, pixels.Color(10, 0, 0));
		display();
	}

	void displayMenu(int menu) {
		clear();
		if (menu == 0) {
			displayFromLut(word, 2);
		} else {
			displayFromLut(hours, menu-1);
		}
		display();
	}

	void displayProgressBar(int value) {
		clearDots();
		if (value > 0 && value < 4) {
			displayFromLut(dots, value);
		}
		display();
	}
};

#endif // __DISPLAY_HPP

