#ifndef __GAME_OF_LIFE_HPP
#define __GAME_OF_LIFE_HPP

struct life_board {
  int cell_size;
  int board_width;
  int board_heigth;
  bool board[SCREEN_WIDTH][SCREEN_HEIGTH];
  bool next_gen[SCREEN_WIDTH][SCREEN_HEIGTH];
  unsigned long last_gen_timestamp;
} life;

int life_CountCellNeighbours(int x, int y) {
  int count = 0;
  int lowX = -1;
  int highX = 2;
  int lowY = -1;
  int highY = 2;

  if (x == 0)
    lowX = 0;
   else if (x == life.board_width-1)
    highX = 1;

   if (y == 0)
    lowY = 0;
   else if (y == life.board_heigth-1)
    highY = 1;

    for (int w=lowX; w<highX; w++) {
      for (int h=lowY; h<highY; h++) {
        if (life.board[x+w][y+h] == 1) {
          count+=1; 
        }
      }
    }

    if (count > 1)
      count -= 1;
 
    return count;
}

void life_NextGen() {
  int survive=0;
  int death=0;
  int birth=0;
  
  for (int w=0; w<life.board_width; w++) {
    for (int h=0; h<life.board_heigth; h++) {
      int neighbours = life_CountCellNeighbours(w,h);
      if (life.board[w][h] == 1) {
        // This is a living cell
        if (neighbours == 2 || neighbours == 3) {
          // Survive
          life.next_gen[w][h] = 1;
          survive++;
        } else {
          // Death
          life.next_gen[w][h] = 0;
          death++;
        }
      } else {
        if (neighbours == 2) {
          // Birth
          life.next_gen[w][h] = 1;
          birth++;
        } else {
          // Death
          life.next_gen[w][h] = 0;
          death++;
        }
      }
    }
  }

  Serial.println((String)"survive:"+survive+", death:"+death+", birth"+birth);
  memcpy(life.board, life.next_gen, sizeof(life.board));
}

void life_display() {
  display.clear();
  int alife=0;
  int dead=0;
  for (int w=0; w<life.board_width; w++) {
    for (int h=0; h<life.board_heigth; h++) {
      if (life.board[w][h] == 1) {
        display.fillRect(w*life.cell_size, h*life.cell_size, life.cell_size, life.cell_size);
      }
    }
  }

  display.display();
}

#endif // __GAME_OF_LIFE_HPP