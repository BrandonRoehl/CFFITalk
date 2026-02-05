#include "../link.h"

bool *cell(Field *self, int x, int y) {
  // If the x or y coordinates are outside the field boundaries they are wrapped
  // toroidally. For instance, an x value of -1 is treated as width-1.
  x %= self->w;
  y %= self->h;
  return self->s[x] + y;
}

extern bool Get(Field const *self, int x, int y) { return *cell(self, x, y); }

extern void Set(Field *self, int x, int y, bool b) { *cell(self, x, y) = b; }
