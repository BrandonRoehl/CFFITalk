#include "../link.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define WIDTH 40
#define HEIGHT 15

Field *newField(int w, int h) {
  bool *xyfield = malloc(w * h * sizeof(bool));
  bool **xfield = malloc(w * sizeof(bool *));
  for (int x = 0; x < w; x++) {
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

void printLife(Life *l) {
  for (int y = 0; y < l->h; y++) {
    for (int x = 0; x < l->w; x++) {
      if (Get(l->a, x, y)) {
        putchar('*');
      } else {
        putchar(' ');
      }
    }
    putchar('\n');
  }
}

void randomizeLife(Life *l) {
  // Generate random starting field
  srand(time(NULL));
  for (int i = 0; i < 50; i++) {
    int x = rand() % l->w;
    int y = rand() % l->h;
    Set(l->a, x, y, true);
  }
}

int main(int argc, char *argv[]) {
  printf("Conway's Game of Life\n");
  Life *l = newLife(WIDTH, HEIGHT);

  randomizeLife(l);

  printf("\0337");
  for (int i = 0; i < 300; i++) {
    Step(l);
    printf("\0338");
    printLife(l);
    usleep(1000000 / 3);
  }
  dropLife(l);
}
