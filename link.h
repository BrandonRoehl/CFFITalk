#ifndef LINK_H
#define LINK_H
#include <stdbool.h>
#include <stdlib.h>

// Field represents a two-dimensional field of cells.
typedef struct {
  // s is a one-dimensional array of booleans representing the state
  // this wraps w*h
  bool **s;
  int32_t w;
  int32_t h;
} Field;

// Life stores the state of a round of Conway's Game of Life.
typedef struct {
  Field *a;
  Field *b;
  int32_t w;
  int32_t h;
} Life;

// Set sets the state of the specified cell to the given value.
extern void Set(Field *self, int x, int y, bool b);

// Get reports whether the specified cell is alive.
// If the x or y coordinates are outside the field boundaries they are
// wrapped toroidally. For instance, an x value of -1 is treated as width-1.
extern bool Get(Field *self, int x, int y);

// Next returns the state of the specified cell at the next time step.
extern bool Next(Field *self, int x, int y);

// Step advances the game by one instant, recomputing and updating all
// cells.
extern void Step(Life *self);

#endif
