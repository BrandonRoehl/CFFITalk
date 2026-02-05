#include "../link.h"

extern void Step(Life *self) {
  // Update the state of the next field (b) from the current field (a).
  for (int x = 0; x < self->w; x++)
    for (int y = 0; y < self->h; y++)
      Set(self->b, x, y, Next(self->a, x, y));

  // Swap fields a and b so that a contains the new state.
  Field *tmp = self->a;
  self->a = self->b;
  self->b = tmp;
}
