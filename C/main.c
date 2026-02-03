#include "../link.h"

#define WIDTH 40
#define HEIGHT 15

int main(int argc, char *argv[]) {
  Field a;
  a.w = WIDTH;
  a.h = HEIGHT;
  bool as[WIDTH * HEIGHT];
  bool *ax[WIDTH];
  for (int x = 0; x < WIDTH; x++)
    ax[x] = as + (x * HEIGHT);
  a.s = ax;

  Field b;
  b.w = WIDTH;
  b.h = HEIGHT;
  bool bs[WIDTH * HEIGHT];
  bool *bx[WIDTH];
  for (int x = 0; x < WIDTH; x++)
    bx[x] = bs + (x * HEIGHT);
  b.s = bx;

  Life l;
  l.w = WIDTH;
  l.h = HEIGHT;
  l.a = &a;
  l.b = &b;

  for (int i = 0; i < 300; i++) {
    Step(&l);
  }
}
