#include "../link.h"
#include <stdlib.h>
#include <unistd.h>

#define WIDTH 40
#define HEIGHT 15

Field *newField(int w, int h) {
  bool *xyfield = malloc(w * h * sizeof(bool));
  bool **xfield = malloc(w * sizeof(bool *));
  for (int x = 0; x < w; w++) {
    xfield[x] = xyfield + (x * h);
  }
  Field *f = malloc(sizeof(Field));
  f->w = w;
  f->h = h;
  f->s = xfield;
  return f;
}

void dropField(Field *f) {
  free(f->s[0]);
  free(f->s);
  free(f);
}

Life *newLife(int w, int h) {
  Life *l = malloc(sizeof(Life));
  l->w = w;
  l->h = h;
  l->a = newField(w, h);
  l->b = newField(w, h);
  return l;
}

void dropLife(Life *l) {
  dropField(l->a);
  dropField(l->b);
  free(l);
}

int main(int argc, char *argv[]) {
  Life *l = newLife(WIDTH, HEIGHT);

  for (int i = 0; i < 50; i++) {
    int x = rand() % l->w;
    int y = rand() % l->h;
    Set(l->a, x, y, true);
  }

  for (int i = 0; i < 300; i++) {
    Step(l);
    usleep(50);
  }
  dropLife(l);
}
