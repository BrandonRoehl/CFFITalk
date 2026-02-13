#include "../link.h"

extern bool Next(const Field *self, int x, int y) {
  // Count the adjacent cells that are alive.
  int alive = 0;
  for (int i = -1; i <= 1; i++)
    for (int j = -1; j <= 1; j++)
      if ((j != 0 || i != 0) && Get(self, x + i, y + j))
        alive++;

  // Return next state according to the game rules:
  //   exactly 3 neighbors: on,
  //   exactly 2 neighbors: maintain current state,
  //   otherwise: off.
  return alive == 3 || alive == 2 && Get(self, x, y);
}
